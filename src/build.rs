#![allow(warnings)]
use fxhash::FxHashMap;
use fxhash::FxHashSet;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::iter;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

pub type Map<K, V> = FxHashMap<K, V>;
pub type Set<K> = FxHashSet<K>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct OperandKind(String);

fn get_vars(sz: usize) -> Vec<Ident> {
    (0..sz).map(|i| format_ident!("x{}", i)).collect()
}

impl OperandKind {
    fn write(&self) -> TokenStream {
        let kind = self.0.as_str();
        match kind {
            "PairIdRefIdRef" => quote! { (ID, ID) },
            "PairIdRefLiteralInteger" => quote! { (ID, u32) },
            "PairLiteralIntegerIdRef" => quote! { (u32, ID) },
            "IdRef" => quote! { ID },
            "IdResult" => quote! { IdResult },
            "IdResultType" => quote! { IdType },
            "IdScope" => quote! { ID },
            "IdMemorySemantics" => quote! { ID },
            "LiteralString" => quote! { String },
            _ if kind.starts_with("Literal") => quote! { u32 },
            _ => {
                let kind = format_ident!("{}", kind);
                quote! { #kind }
            }
        }
    }
    pub fn is_trivial(&self, lookup: &mut Map<OperandKind, bool>, operands: &[Operand]) -> bool {
        if let Some(trivial) = lookup.get(self) {
            return *trivial;
        }
        let kind = self.0.as_str();
        let re = match kind {
            "PairIdRefIdRef"
            | "PairIdRefLiteralInteger"
            | "PairLiteralIntegerIdRef"
            | "IdRef"
            | "IdResult"
            | "IdResultType"
            | "IdScope"
            | "IdMemorySemantics" => true,
            "LiteralString" => false,
            _ if kind.starts_with("Literal") => true,
            _ => operands
                .iter()
                .find(|o| o.kind == *self)
                .unwrap()
                .is_trivial(lookup, operands),
        };
        lookup.insert(self.clone(), re);
        re
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Parameter {
    kind: OperandKind,
}

impl Parameter {
    pub fn write(&self) -> TokenStream {
        self.kind.write()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Enumerant {
    enumerant: String,
    value: Value,
    #[serde(default)]
    version: String,
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    capabilities: Vec<String>,
    #[serde(default)]
    doc: String,
    #[serde(default)]
    parameters: Vec<Parameter>,
}

impl Enumerant {
    pub fn value(&self) -> u32 {
        match &self.value {
            Value::String(s) => u32::from_str_radix(&s[2..], 16).expect(s),
            Value::Number(n) => n.as_u64().unwrap() as u32,
            _ => panic!(),
        }
    }

    pub fn opname(&self) -> proc_macro2::Ident {
        let p = if self.enumerant.chars().nth(0).unwrap().is_digit(10) {
            "_"
        } else {
            ""
        };
        format_ident!("{}{}", p, &self.enumerant)
    }

    fn is_dupe(&self, val_set: &mut Set<u32>) -> bool {
        let value = self.value();
        if val_set.contains(&value) {
            return true;
        }
        val_set.insert(value);
        false
    }

    pub fn write(&self) -> TokenStream {
        let opcode = Literal::u32_unsuffixed(self.value());
        let opname = self.opname();
        let operands = if self.parameters.is_empty() {
            quote! {}
        } else {
            let operands = self.parameters.iter().map(Parameter::write);
            quote! { (#(#operands),*) }
        };
        quote! { #opname #operands = #opcode, }
    }

    fn is_trivial(&self, lookup: &mut Map<OperandKind, bool>, operands: &[Operand]) -> bool {
        self.parameters
            .iter()
            .fold(true, |c, p| c && p.kind.is_trivial(lookup, operands))
    }

    fn write_match(&self) -> TokenStream {
        if self.parameters.is_empty() {
            return quote! {};
        }
        let name = self.opname();
        let args = &get_vars(self.parameters.len());
        quote! {
          #name(#(#args,)*) => {#(#args.write_word(sink);)*},
        }
    }

    fn write_read(&self) -> TokenStream {
        let name = self.opname();
        let opc = Literal::u32_unsuffixed(self.value());
        if self.parameters.is_empty() {
            return quote! { #opc => #name, };
        }
        let args = &get_vars(self.parameters.len());

        quote! {
          #opc => {
            #(let #args = Asm::read_word(chunk, idx); )*
            #name(#(#args,)*)
          },
        }
    }

    fn write_test(&self) -> TokenStream {
        let opcode = Literal::u32_unsuffixed(self.value());
        let opname = self.opname();
        let args = &get_vars(self.parameters.len());

        let args2 = if self.parameters.is_empty() {
            quote! {#opname}
        } else {
            quote! { #opname(#(#args,)*) }
        };

        quote! {
          unsafe {
            #(let #args = <_>::default(); )*
            let variant = #args2;
            let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
            assert_eq!(disc, #opcode);
          }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Operand {
    category: String,
    kind: OperandKind,
    #[serde(default)]
    enumerants: Vec<Enumerant>,
    #[serde(default)]
    bases: Vec<OperandKind>,
}

impl Operand {
    fn is_bitfield(&self) -> bool {
        self.category == "BitEnum"
    }

    fn is_trivial(&self, lookup: &mut Map<OperandKind, bool>, operands: &[Operand]) -> bool {
        if let Some(trivial) = lookup.get(&self.kind) {
            return *trivial;
        }
        let re = self
            .enumerants
            .iter()
            .fold(true, |c, p| c && p.is_trivial(lookup, operands))
            && self
                .bases
                .iter()
                .fold(true, |c, p| c && p.is_trivial(lookup, operands));
        lookup.insert(self.kind.clone(), re);
        re
    }

    fn write_impl(&self) -> TokenStream {
        if self.enumerants.is_empty() {
            return quote! {};
        }
        let vars: Vec<_> = (0..30usize).map(|i| format_ident!("x{}", i)).collect();
        let matcher = self.enumerants.iter().map(|e| e.write_match());
        let reader = self.enumerants.iter().map(|e| e.write_read());

        let name = format_ident!("{}", self.kind.0);
        quote! {
          impl #name {
            pub fn opcode(&self) -> u32 {
              unsafe { std::mem::transmute_copy(self) }
            }
          }

          impl Asm for #name {
            fn write_word(&self, sink: &mut Vec<u32>) {
              use #name ::*;
              sink.push(self.opcode());
              match self {
                #(#matcher)*
                what => panic!("{:?}", what),
              }
            }

            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              use #name ::*;
              *idx += 1;
              match chunk[*idx as usize - 1] {
                #(#reader)*
                what => panic!("{:?}", what),
              }
            }
          }
        }
    }

    fn init(&mut self) {
        let mut val_set = Set::default();
        let mut enums = vec![];
        std::mem::swap(&mut enums, &mut self.enumerants);
        self.enumerants = enums
            .into_iter()
            .filter(|e| !e.is_dupe(&mut val_set))
            .collect();
    }

    fn write(&self, lookup: &mut Map<OperandKind, bool>, operands: &[Operand]) -> TokenStream {
        if self.enumerants.is_empty() {
            return quote! {};
        }
        let name = format_ident!("{}", self.kind.0);
        let enumerants = self.enumerants.iter().map(|e| e.write());

        let mut derive = vec!["Default", "Debug", "Clone", "Hash", "Eq", "PartialEq"];
        if self.is_trivial(lookup, operands) {
            derive.push("Copy");
        }
        let derive = derive.iter().map(|d| format_ident!("{}", d));

        quote! {
          #[repr(u32)]
          #[derive(#(#derive),*)]
          pub enum #name {
            #(#enumerants)*
            #[default]
            INVALID = u32::MAX,
          }
        }
    }

    fn write_test(&self) -> TokenStream {
        if self.enumerants.is_empty() {
            return quote! {};
        }
        let name = format_ident!("{}", self.kind.0);
        let fn_name = format_ident!("___test___{}", self.kind.0);
        let tests = self.enumerants.iter().map(|e| e.write_test());

        quote! {
          #[test]
          pub fn #fn_name() {
            use #name ::*;
            use std::mem::*;
            #(#tests)*
          }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct OperandRef {
    kind: OperandKind,
    #[serde(default)]
    quantifier: String,
    #[serde(default)]
    name: String,
}

impl OperandRef {
    fn write(&self) -> TokenStream {
        let kind = self.kind.write();
        match self.quantifier.as_str() {
            "?" => quote! { Option<#kind> },
            "*" => quote! { Vec<#kind> },
            "" => quote! { #kind },
            _ => unreachable!(),
        }
    }

    fn is_trivial(&self, lookup: &mut HashMap<OperandKind, bool>) -> bool {
        if let Some(trivial) = lookup.get(&self.kind) {
            return *trivial;
        }
        let re = if self.quantifier == "*" {
            false
        } else {
            match self.kind.0.as_str() {
                "LiteralString" => false,
                _ => true,
            }
        };
        lookup.insert(self.kind.clone(), re);
        re
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Instruction {
    opname: String,
    #[serde(default)]
    class: String,
    opcode: u32,
    #[serde(default)]
    operands: Vec<OperandRef>,
    #[serde(default)]
    version: String,
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    capabilities: Vec<String>,
}

impl Instruction {
    pub fn write(&self) -> TokenStream {
        if self.is_specop() {
            return quote! {
              OpSpecConstantOp(IdType, IdResult, Box<Opcode>) = 52,
            };
        }
        let opcode = Literal::u32_unsuffixed(self.opcode);
        let opname = format_ident!("{}", &self.opname);
        let operands = self.operands.iter().map(OperandRef::write);
        let operands = if self.operands.is_empty() {
            quote! {}
        } else {
            quote! { (#(#operands),*) }
        };

        quote! { #opname #operands = #opcode, }
    }

    fn is_dupe(&self, val_set: &mut Set<u32>) -> bool {
        if val_set.contains(&self.opcode) {
            return true;
        }
        val_set.insert(self.opcode);
        false
    }

    fn is_specop(&self) -> bool {
        self.opname.as_str() == "OpSpecConstantOp"
    }

    fn has_type_and_id(&self) -> bool {
        match &self.operands[..] {
            [ty, id, ..] => &ty.kind.0 == "IdResultType" && &id.kind.0 == "IdResult",
            _ => false,
        }
    }

    pub fn write_impl(&self) -> TokenStream {
        let opcode = Literal::u32_unsuffixed(self.opcode);
        let opname = format_ident!("{}", &self.opname);
        let args = &get_vars(self.operands.len());
        if self.operands.is_empty() {
            return quote! {  #opname => (), };
        }

        if self.is_specop() {
            return quote! {
              #opname(_,_,specop) => specop.write_word(sink),
            };
        }

        quote! {
          #opname(#(#args,)*) => {#(#args.write_word(sink);)*},
        }
    }

    pub fn specop_read_impl(&self) -> TokenStream {
        let opcode = Literal::u32_unsuffixed(self.opcode);
        let opname = format_ident!("{}", &self.opname);
        let args = &get_vars(self.operands.len())[2..];
        quote! {
          #opcode => {
            #(let #args = Asm::read_word(chunk, idx); )*
            #opname(ty, id, #(#args,)*)
          }
        }
    }

    pub fn read_impl(&self) -> TokenStream {
        let opcode = Literal::u32_unsuffixed(self.opcode);
        let opname = format_ident!("{}", &self.opname);
        let args = &get_vars(self.operands.len());

        if self.operands.is_empty() {
            return quote! { #opcode => #opname, };
        }

        if self.is_specop() {
            return quote! {
              #opcode => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Opcode::read_as_spec_op(x0, x1, chunk, idx);
                #opname(x0, x1, Box::new(x2))
              }
            };
        }

        quote! {
          #opcode => {
            #(let #args = Asm::read_word(chunk, idx); )*
            #opname(#(#args,)*)
          }
        }
    }

    fn write_test(&self) -> TokenStream {
        let opcode = Literal::u32_unsuffixed(self.opcode);
        let opname = format_ident!("{}", &self.opname);
        let args = &get_vars(self.operands.len());

        let args2 = if self.operands.is_empty() {
            quote! {#opname}
        } else {
            quote! { #opname(#(#args,)*) }
        };

        quote! {
          unsafe {
            #(let #args = <_>::default();)*
            let variant = #args2;
            let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
            assert_eq!(disc, #opcode);
          }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Grammar {
    magic_number: String,
    major_version: u32,
    minor_version: u32,
    revision: u32,
    instructions: Vec<Instruction>,
    #[serde(default, rename(deserialize = "operand_kinds"))]
    operands: Vec<Operand>,
}

impl Grammar {
    fn init(&mut self) {
        let mut val_set = Set::default();
        let mut instructions = vec![];
        std::mem::swap(&mut instructions, &mut self.instructions);
        self.instructions = instructions
            .into_iter()
            .filter(|e| !e.is_dupe(&mut val_set))
            .collect();

        self.operands.iter_mut().for_each(Operand::init);

        for op in &self.operands {
            if op.is_bitfield() {
              println!("{}", op.kind.0);
            }
        }
        panic!();
    }

    fn write(&self) -> TokenStream {
        let mut trivials = Map::default();

        let operands = self
            .operands
            .iter()
            .map(|o| o.write(&mut trivials, &self.operands));
        let operand_impls = self.operands.iter().map(Operand::write_impl);
        let instructions = self.instructions.iter().map(|e| e.write());
        let instruction_writers = self.instructions.iter().map(Instruction::write_impl);
        let instruction_readers = self.instructions.iter().map(Instruction::read_impl);
        let specop_readers = self
            .instructions
            .iter()
            .filter(|i| i.has_type_and_id())
            .map(Instruction::specop_read_impl);

        quote! {

          pub use Opcode::*;
          #[repr(u32)]
          #[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
          pub enum Opcode {#[default] #(#instructions)*}
          #(#operands)*

          #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
          pub struct ID(pub u32);

          pub type IdResult = ID;
          pub type IdType = ID;

          pub trait Asm {
            fn write_word(&self, sink: &mut Vec<u32>);
            fn read_word(chunk: &[u32], idx: &mut usize) -> Self;
          }

          impl Opcode {
            pub fn opcode(&self) -> u32 {
              unsafe { std::mem::transmute_copy(self) }
            }

            fn read_as_spec_op(ty: IdType, id: IdResult, chunk: &[u32],  idx: &mut usize) -> Self {
              use Opcode::*;
              let i = *idx as usize;
              let mask = u16::MAX as u32;;
              let opc = chunk[i] & mask;
              *idx += 1;
              match opc {
                #(#specop_readers,)*
                what => panic!("{:?}", what),
              }
            }
          }

          impl Asm for Opcode {
            fn write_word(&self, sink: &mut Vec<u32>) {
              let mark = sink.len();
              sink.push(self.opcode());
              match self {
                #(#instruction_writers)*
                what => panic!("{:?}", what),
              }
              sink[mark] |= ((sink.len() - mark) as u32) << 16;
            }

            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              use Opcode::*;
              let len = (chunk[*idx] >> 16) as usize;
              let opc = chunk[*idx] & 0xffff;
              let chunk = &chunk[..*idx + len];
              let mark = *idx;
              *idx += 1;
              let re = match opc {
                #(#instruction_readers)*
                what => panic!("{:?}", what),
              };
              assert_eq!(*idx - mark, len);
              re
            }
          }

          #(#operand_impls)*

          impl<T: Asm> Asm for Option<T> {
            fn write_word(&self, sink: &mut Vec<u32>) {
              self.as_ref().map(|t| t.write_word(sink));
            }

            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              if *idx < chunk.len() {
                Some(T::read_word(chunk, idx))
              } else {
                None
              }
            }
          }

          impl<T: Asm, U: Asm> Asm for (T, U) {
            fn write_word(&self, sink: &mut Vec<u32>) {
              self.0.write_word(sink);
              self.1.write_word(sink);
            }

            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              let t = T::read_word(chunk, idx);
              let u = U::read_word(chunk, idx);
              (t, u)
            }
          }

          impl<T: Asm> Asm for Box<T> {
            fn write_word(&self, sink: &mut Vec<u32>) {
              self.as_ref().write_word(sink);
            }

            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              Box::new(T::read_word(chunk, idx))
            }
          }

          impl<T: Asm> Asm for Vec<T> {
            fn write_word(&self, sink: &mut Vec<u32>) {
              self.iter().for_each(|t| t.write_word(sink));
            }

            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              let mut re = vec![];
              while *idx < chunk.len() {
                re.push(T::read_word(chunk, idx));
              }
              re
            }
          }

          impl Asm for u32 {
            fn write_word(&self, sink: &mut Vec<u32>) {
              sink.push(*self);
            }

            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              *idx += 1;
              chunk[*idx as usize - 1]
            }
          }

          impl Asm for String {
            fn write_word(&self, sink: &mut Vec<u32>) {
              let mark = sink.len();
              let strlen = (self.len() >> 2) + 1;
              sink.resize(mark + strlen, 0);
              unsafe {
                  std::ptr::copy_nonoverlapping(
                      self.as_ptr(),
                      sink.as_mut_ptr().offset(mark as isize) as *mut u8,
                      self.len(),
                  );
              }
            }

            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              let chunk = &chunk[*idx as usize..];
              let mut len = 0;
              'outer: for u in chunk {
                for u in u.to_le_bytes().iter() {
                  if *u == 0 {
                    break 'outer;
                  }
                  len += 1;
                }
              }
              let offset = (len >> 2) + 1;
              *idx += offset;
              unsafe {
                let s = std::slice::from_raw_parts((chunk.as_ptr() as * const u8), len as usize);
                std::str::from_utf8(s).unwrap().to_owned()
              }
            }
          }

          impl Asm for ID {
            fn write_word(&self,sink: &mut Vec<u32>) {
              sink.push(self.0);
            }
            fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
              let id = ID(chunk[*idx as usize]);
              *idx += 1;
              id
            }
          }
        }
    }

    pub fn tests(&self) -> TokenStream {
        let tester = self.instructions.iter().map(|i| i.write_test());
        let tester2 = self.operands.iter().map(|i| i.write_test());

        quote! {
          use crate::opcode::*;
          #[test]
          pub fn ___test___Opcode() {
            use Opcode::*;
            use std::mem::*;
            #(#tester)*
          }
          #(#tester2)*
        }
    }
}

fn main() {
    let mut grammar: Grammar = serde_json::from_str(
        &std::fs::read_to_string("SPIRV-Headers/include/spirv/unified1/spirv.core.grammar.json")
            .unwrap(),
    )
    .unwrap();
    grammar.init();

    std::fs::write("src/opcode.rs", rustfmt(&grammar.write().to_string()));
    std::fs::write("src/tests.rs", rustfmt(&grammar.tests().to_string()));

    println!("cargo:rerun-if-changed=src/build.rs");
    println!("cargo:rerun-if-changed=src/opcode.rs");
    println!("cargo:rerun-if-changed=src/tests.rs");
}

fn rustfmt(source: &str) -> String {
    let mut cmd = Command::new("rustfmt");

    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let source = source.to_owned();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(source.as_bytes());
        source
    });

    let mut output = vec![];
    io::copy(&mut child_stdout, &mut output).unwrap();

    let status = child.wait().unwrap();
    let source = stdin_handle.join().expect(
        "The thread writing to rustfmt's stdin doesn't do \
             anything that could panic",
    );

    String::from_utf8(output).unwrap()
}

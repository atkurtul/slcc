#![allow(non_camel_case_types, dead_code, non_upper_case_globals)]
#![allow(warnings)]
#![feature(arbitrary_enum_discriminant, core_intrinsics)]
pub use crate::glsl450;
pub use crate::opcode;
pub use crate::opencl100;

use std::{collections::BTreeMap, env::Args, fs::read};

pub type Map<K, V> = BTreeMap<K, V>;
pub use opcode::*;
pub type Sstr = &'static str;

pub fn read_spirv(src: &str) -> Box<[u32]> {
    let mut src = std::fs::read(src).unwrap();
    src.shrink_to_fit();
    let mut src = unsafe {
        let v = Vec::from_raw_parts(src.as_ptr() as *mut u32, src.len() >> 2, src.len() >> 2);
        std::mem::forget(src);
        v
    };

    if src[0] != 0x07230203 {
        for word in src.iter_mut() {
            *word = word.swap_bytes();
        }
    }

    assert_eq!(src[0], 0x07230203);

    src.into_boxed_slice()
}

pub fn write_spirv(path: &str, src: &[u32]) {
    let src = unsafe { std::slice::from_raw_parts(src.as_ptr() as *const u8, src.len() * 4) };
    std::fs::write(path, src);
}

impl Environ for Module {
    fn insert_id(&mut self, id: ResultID) {
        let re = self.instr.len();
        self.ids.insert(id, re);
    }

    fn insert_op(&mut self, opc: Opcode) -> Opcode {
        self.instr.push(opc.clone());
        opc
    }
}

#[derive(Default, Debug, Clone)]
pub struct Header {
    pub magic: u32,
    pub version: u32,
    pub generator: u32,
    pub bound: u32,
    pub schema: u32,
}

impl Header {
    pub fn new(bin: &[u32]) -> Header {
        Header {
            magic: bin[0],
            version: bin[1],
            generator: bin[2],
            bound: bin[3],
            schema: bin[4],
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Module {
    pub src: Box<[u32]>,
    pub header: Header,
    pub instr: Vec<Opcode>,
    pub ids: Map<ResultID, usize>,
}

impl Module {
    pub fn new(src: &str) -> Module {
        let src = read_spirv(src);
        Module::new_code(Header::new(&src[..5]), &src[5..])
    }

    pub fn new_code(header: Header, bin: &[u32]) -> Module {
        let mut module = Module::default();
        module.header = header;

        let mut idx = 0;

        while idx < bin.len() {
            Opcode::read_word(bin, &mut module, &mut idx);
        }

        module
    }

    pub fn finalize(&mut self, bound: u32) {
        self.header.magic = 0x07230203;
        self.header.version = 0x00010300;
        self.header.generator = 0;
        self.header.bound = bound;
        self.header.schema = 0;

        let mut bin = vec![
            self.header.magic,
            self.header.version,
            self.header.generator,
            self.header.bound,
            self.header.schema,
        ];

        for inst in self.instr.iter() {
            inst.write_word(self, &mut bin);
        }
        self.src = bin.into_boxed_slice()
    }
}

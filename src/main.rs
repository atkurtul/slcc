#![feature(arbitrary_enum_discriminant, box_patterns)]
#![allow(non_camel_case_types, unused_imports, unused_variables, unused_parens)]

pub mod expr;
pub mod glsl450;
pub mod lang;
pub mod opcode;
pub mod opencl100;
pub mod spirv;

pub use expr::*;

// mod lang {
//     include!(concat!(env!("OUT_DIR"), "/lang.rs"));
// }

use expr::*;
use std::collections::HashMap;

use std::env;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    file: String,

    /// Number of times to greet
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let parser = lang::xLangParser::new();
    let src = std::fs::read_to_string(&args.file).unwrap();
    let ast = parser.parse(&src).unwrap();
    let mut ctx = Context::new();

    for (kernel, name, args, out, stats) in ast {
        ctx.create_func(kernel.unwrap_or((1, 1, 1)), name, args, out, stats);
    }

    let module = ctx.into_module();
    write_spirv(&args.output, &*module.src);
}

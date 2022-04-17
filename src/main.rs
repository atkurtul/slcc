#![feature(arbitrary_enum_discriminant, derive_default_enum, box_patterns)]
#![allow(
    warnings,
    non_camel_case_types,
    unused_imports,
    unused_variables,
    unused_parens
)]

use clap::Parser;
use std::env;

mod test_asm;
mod tests;

pub mod context;
pub mod expr;
pub mod function;
pub mod lang;
pub mod module;
pub mod opcode;
pub mod types;
pub mod util;

use context::*;
use expr::*;
use function::*;
use lang::*;
use module::*;
use opcode::*;
use types::*;
use util::*;

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
    let mut ctx = Context::default();
    let ast = parser.parse(&mut ctx, &src).unwrap();
    ctx.go();
}

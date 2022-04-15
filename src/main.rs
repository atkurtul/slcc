#![feature(arbitrary_enum_discriminant, derive_default_enum)]
#![allow(warnings)]
use fxhash::FxHashMap;
use fxhash::FxHashSet;

pub type Map<K, V> = FxHashMap<K, V>;
pub type Set<K> = FxHashSet<K>;

pub mod opcode;
pub use opcode::*;
mod tests;
mod test_asm;

fn main() {}

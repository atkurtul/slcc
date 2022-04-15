#![feature(arbitrary_enum_discriminant, derive_default_enum)]
#![allow(warnings)]
pub mod opcode;
pub use opcode::*;

pub fn read_spirv(src_file: &str) -> Box<[u32]> {
    let mut src = std::fs::read(src_file).unwrap();
    src.shrink_to_fit();
    let mut src = unsafe {
        let v = Vec::from_raw_parts(src.as_ptr() as *mut u32, src.len() >> 2, src.len() >> 2);
        std::mem::forget(src);
        v
    };
    for word in src.iter_mut() {
        *word = word.swap_bytes();
    }

    if src[0] != 0x07230203 {
        for word in src.iter_mut() {
            *word = word.swap_bytes();
        }
    }

    assert_eq!(src[0], 0x07230203);

    src.into_boxed_slice()
}

pub fn read_ops(src: &[u32]) -> Box<[Opcode]> {
    let mut idx = 0;
    let mut ops = vec![];
    while idx < src.len() {
        let opc = Opcode::read_word(src, &mut idx);
        ops.push(opc);
    }
    ops.into_boxed_slice()
}

pub fn write_ops(ops: &[Opcode]) -> Box<[u32]> {
    let mut bin = vec![];
    for op in ops {
        op.write_word(&mut bin);
    }
    bin.into_boxed_slice()
}
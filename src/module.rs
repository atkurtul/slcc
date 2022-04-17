use crate::*;

pub fn write_spirv(path: &str, src: &[u32]) {
    let src = unsafe { std::slice::from_raw_parts(src.as_ptr() as *const u8, src.len() * 4) };
    std::fs::write(path, src);
}

#[derive(Default, Debug, Clone)]
pub struct Module {
    pub magic: u32,
    pub version: u32,
    pub generator: u32,
    pub bound: u32,
    pub schema: u32,
    pub ops: Vec<Opcode>,
    pub req: ModuleRequirements,
}

impl Module {
    pub fn insert_op(&mut self, op: Opcode) {
        self.ops.push(op);
    }

    pub fn finalize(&mut self, bound: u32) {
        self.magic = 0x07230203;
        self.version = 0x00010300;
        self.generator = 0;
        self.bound = bound;
        self.schema = 0;
        let mut bin = vec![
            self.magic,
            self.version,
            self.generator,
            self.bound,
            self.schema,
        ];

        for op in self.ops.iter() {
            op.write_word(&mut bin, &mut self.req);
        }

    }
}

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
    let mut req = <_>::default();
    for op in ops {
        op.write_word(&mut bin, &mut req);
    }
    bin.into_boxed_slice()
}

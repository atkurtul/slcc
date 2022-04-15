pub use crate::opcode::*;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

fn visit_dirs(dir: &Path, cb: fn(&str)) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            visit_dirs(&path, cb);
        }
    } else {
        cb(dir.as_os_str().to_str().unwrap());
    }
}


fn test_bin(src: &str) {
    println!("Testing {}", src);
    let mut src = slcc::read_spirv(src);
    let mut ops = slcc::read_ops(&src[5..]);
    let mut bin = slcc::write_ops(&ops);
    assert_eq!(ops, slcc::read_ops(&bin));
}

#[test]
fn test_asm() {
    visit_dirs(Path::new("bin/shaders"), test_bin);
}

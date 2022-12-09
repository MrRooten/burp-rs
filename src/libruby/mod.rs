use std::fs;

use rutie::{VM, eval};
pub mod utils;
pub mod rb_main;
pub mod http;
pub fn test_ruby(f: &str) {
    VM::init();
    VM::init_loadpath();
    let f = fs::read_to_string(f).unwrap();
    eval!(&f);
}

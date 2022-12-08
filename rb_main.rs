use std::thread::{self, spawn, JoinHandle};

use rutie::{Fixnum, Thread};

use crate::modules::{active::ruby_scan::RBModule, get_next_to_scan, IActive};

use super::utils::rb_init;

fn get_modules() -> Vec<Box<dyn IActive>> {
    let mut result: Vec<Box<dyn IActive>> = vec![];
    result.push(Box::new(RBModule::new("./active/test1.rs").unwrap()));
    result.push(Box::new(RBModule::new("./active/test2.rs").unwrap()));
    result
}

pub fn ruby_thread() -> JoinHandle<()> {
    let t = spawn(|| {
        rb_init();
        let modules = get_modules();
        let index = get_next_to_scan();
        for module in modules {
            Thread::new(|| {
                module.passive_run(index);
                Fixnum::new(0)
            });
        }
    });
    t
}

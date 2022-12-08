use std::thread::{self, spawn, JoinHandle};

use rutie::{Fixnum, Thread, Object};

use crate::modules::{active::ruby_scan::RBModule, get_next_to_scan, IActive};

use super::utils::rb_init;

fn get_modules() -> Vec<Box<dyn IActive>> {
    let mut result: Vec<Box<dyn IActive>> = vec![];
    result.push(Box::new(RBModule::new("./active/test1.rb").unwrap()));
    result.push(Box::new(RBModule::new("./active/test2.rb").unwrap()));
    result.push(Box::new(RBModule::new("./active/test3.rb").unwrap()));
    result.push(Box::new(RBModule::new("./active/test3.rb").unwrap()));
    result
}

pub fn ruby_thread() -> JoinHandle<()> {
    let t = spawn(|| {
        rb_init();
        let modules = get_modules();
        let index = get_next_to_scan();
        let mut s = vec![];
        for module in modules {
            let thread = Thread::new(|| {
                module.passive_run(index);
                Fixnum::new(0)
            });

            s.push(thread);
        }

        for thread in s {
            thread.protect_send("join", &[]);
        }
    });
    t
}

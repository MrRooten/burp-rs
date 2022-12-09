use std::{
    fs,
    sync::mpsc,
    thread::{ spawn, JoinHandle},
};

use rutie::{Fixnum, Object, Thread};

use crate::{
    cmd::handlers::{SCAN_RECEIVER, SCAN_SENDER},
    modules::{
        active::{ ruby_scan::RBModule},
        get_next_to_scan, IActive,
    },
};

use super::utils::rb_init;

fn get_modules(dir: &str) -> Vec<Box<dyn IActive>> {
    let mut result: Vec<Box<dyn IActive>> = vec![];
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let s = path.unwrap().path().to_str().unwrap().to_string();
        if s.ends_with(".rb") {
            result.push(Box::new(RBModule::new(&s).unwrap()));
        }
    }
    result
}

pub fn ruby_thread() -> JoinHandle<()> {
    unsafe {
        if SCAN_SENDER.is_none() {
            let (tx, rx) = mpsc::channel::<u32>();
            SCAN_SENDER = Some(tx);
            SCAN_RECEIVER = Some(rx);
        }
    }
    let t = spawn(|| {
        let _ = rb_init();
        let modules = get_modules("./active/");
        let index = get_next_to_scan();
        let mut s = vec![];
        for module in modules {
            let thread = Thread::new(|| {
                let _ = module.passive_run(index);
                Fixnum::new(0)
            });

            s.push(thread);
        }

        for thread in s {
            let _ = thread.protect_send("join", &[]);
        }
    });
    t
}

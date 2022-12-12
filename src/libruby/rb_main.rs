use std::{
    fs,
    sync::mpsc,
    thread::{spawn, JoinHandle},
};

use log::error;
use rutie::{Fixnum, Object, Thread};

use crate::{
    cmd::handlers::{SCAN_RECEIVER, SCAN_SENDER},
    modules::{active::ruby_scan::RBModule, get_next_to_scan, IActive, ModuleMeta, GLOB_POCS, get_will_run_pocs},
};

use super::utils::rb_init;

fn get_ruby_modules(dir: &str) -> Vec<Box<dyn IActive>> {
    let mut result: Vec<Box<dyn IActive>> = vec![];
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let s = path.unwrap().path().to_str().unwrap().to_string();
        if s.ends_with(".rb") {
            let module = Box::new(RBModule::new(&s).unwrap());
            unsafe {
                match module.metadata() {
                    Some(s) => {
                        GLOB_POCS.push(s.clone());
                    },
                    None => {

                    }
                };
            }
            result.push(module);
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
        loop {
            let modules = get_ruby_modules("./active/");
            let will_run_modules = get_will_run_pocs();
            let index = get_next_to_scan();
            let mut s = vec![];
            for module in modules {
                let meta = match module.metadata() {
                    Some(o) => o,
                    None => {
                        continue;
                    }
                };
                if will_run_modules.contains(&meta.get_name().to_string()) == false{
                    continue;
                }
                let thread = Thread::new(|| {
                    let v = module.passive_run(index);
                    match v {
                        Ok(o) => {},
                        Err(e) => {
                            error!("{:?}",e);
                        }
                    }
                    Fixnum::new(0)
                });

                s.push(thread);
            }

            for thread in s {
                let _ = thread.protect_send("join", &[]);
            }
        }
    });
    t
}

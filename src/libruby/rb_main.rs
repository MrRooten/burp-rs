use std::{
    fs,
    sync::mpsc::{self, Sender},
    thread::{spawn, JoinHandle}, collections::HashMap,
};

use log::{error, info};
use rutie::{Fixnum, Object, Thread, Hash};

use crate::{
    cmd::handlers::{SCAN_RECEIVER, SCAN_SENDER},
    modules::{
        active::ruby_scan::RBModule, get_next_to_scan, get_will_run_pocs, IActive, GLOB_POCS,
    },
    st_error,
    utils::STError,
};

use super::utils::rb_init;
pub static mut MODULE_INDEX: usize = 0;
pub static mut RUBY_MODULES: Vec<RBModule> = Vec::new();

pub fn update_modules() {
    unsafe {
        let len = RUBY_MODULES.len();
        for i in 0..len {
            if RUBY_MODULES[i].is_change() {
                info!("{:?} file has changed",RUBY_MODULES[i].metadata());
                let _ = RUBY_MODULES[i].update();
            }
        }
    }
}

pub fn update_and_get_modules(dir: &str) -> &Vec<RBModule> {
    unsafe {
        RUBY_MODULES.clear();
    }
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let s = path.unwrap().path().to_str().unwrap().to_string();
        if s.ends_with(".rb") {
            let module = RBModule::new(&s).unwrap();
            unsafe {
                match module.metadata() {
                    Some(s) => {
                        GLOB_POCS.push(s.clone());
                    }
                    None => {}
                };
            }
            unsafe {
                RUBY_MODULES.push(module);
            }
        }
    }

    unsafe { &RUBY_MODULES }
}

pub static mut RUBY_COMMAND_SENDER: Option<std::sync::mpsc::Sender<String>> = None;
pub static mut RUBY_COMMAND_RECEIVER: Option<std::sync::mpsc::Receiver<String>> = None;
pub fn get_command() -> String {
    unsafe {

        let receiver = match &RUBY_COMMAND_RECEIVER {
            Some(s) => s,
            None => return "error".to_string(),
        };
        let command = receiver.recv();

        let command = match command {
            Ok(o) => o,
            Err(e) => {
                return "error".to_string();
            }
        };
        return command.clone();
    }
}

pub fn send_command(command: &str) -> Result<(), STError> {
    unsafe {
        let sender = match &RUBY_COMMAND_SENDER {
            Some(s) => s,
            None => {
                return Err(STError::new("Sender is not available"));
            }
        };
        match sender.send(command.to_string()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(st_error!(e));
            }
        }
    }
}

pub fn ruby_thread() -> JoinHandle<()> {
    unsafe {
        if SCAN_SENDER.is_none() {
            let (tx, rx) = mpsc::channel::<u32>();
            SCAN_SENDER = Some(tx);
            SCAN_RECEIVER = Some(rx);
        }

        if RUBY_COMMAND_SENDER.is_none() {
            let (sender, receiver) = std::sync::mpsc::channel::<String>();
            RUBY_COMMAND_SENDER = Some(sender);
            RUBY_COMMAND_RECEIVER = Some(receiver);
        }
    }
    let t = spawn(|| {
        let _ = rb_init();
        let modules = update_and_get_modules("./active/");
        // Thread::new(|| {
        //     loop {
        //         let command = get_command();
        //         if command.eq("reload") {
        //             let _ = update_and_get_modules("./active/");
        //         }
        //     }
        //     Fixnum::new(0)
        // });
        loop {
            let will_run_modules = get_will_run_pocs();
            let index = get_next_to_scan();
            update_modules();
            let mut s = vec![];
            for module in modules {
                let meta = match module.metadata() {
                    Some(o) => o,
                    None => {
                        continue;
                    }
                };
                //if will_run_modules.contains(&meta.get_name().to_string()) == false{
                //    continue;
                //}
                let thread = Thread::new(|| {
                    let v = module.passive_run(index);
                    match v {
                        Ok(o) => {}
                        Err(e) => {
                            error!("{:?}", e);
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

use std::{
    collections::HashMap,
    fs,
    sync::mpsc::{self},
    thread::{spawn, JoinHandle},
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use log::{error, info};
use rutie::{eval, Fixnum, Object, Thread};

use crate::{
    cmd::handlers::{SCAN_RECEIVER, SCAN_SENDER},
    libruby::{http::thread::rb_http_thread, utils::rb_init},
    modules::{
        active::ruby_scan::RBModule, get_next_to_scan, get_will_run_pocs, IActive, ModuleType,
        GLOB_POCS,
    },
    st_error,
    utils::STError,
};

pub static mut MODULE_INDEX: usize = 0;
pub static mut MODULES: Vec<Box<dyn IActive + Sync>> = Vec::new();
static mut RUNING_MODULES: Option<HashMap<u32, RunningModuleWrapper>> = None;
static mut WILL_RELOAD: bool = false;

pub fn set_reload() {
    unsafe {
        WILL_RELOAD = true;
    }
}

pub fn unset_reload() {
    unsafe {
        WILL_RELOAD = false;
    }
}

#[derive(Clone, PartialEq)]
pub enum RunningState {
    RUNNING,
    DEAD,
    EXCEPTION,
}

#[derive(Clone)]
pub struct RunningModuleWrapper {
    name: String,
    date: DateTime<Local>,
    index: u32,
    state: RunningState,
    args: u32,
    cost: u128,
}

impl RunningModuleWrapper {
    pub fn new(name: &str, index: u32, args: u32) -> Self {
        RunningModuleWrapper {
            name: name.to_string(),
            date: Local::now(),
            index: index,
            state: RunningState::RUNNING,
            args: args,
            cost: 0,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_starttime(&self) -> &DateTime<Local> {
        &self.date
    }

    pub fn get_state(&self) -> &RunningState {
        &self.state
    }

    pub fn get_state_colored(&self) -> ColoredString {
        if self.state.eq(&RunningState::RUNNING) {
            return "RUNNING".to_string().green();
        } else if self.state.eq(&RunningState::DEAD) {
            return "DEAD".to_string().red();
        } else if self.state.eq(&RunningState::EXCEPTION) {
            return "EXCEPTION".to_string().yellow();
        }

        return "".to_string().red();
    }

    pub fn get_args(&self) -> u32 {
        self.args
    }

    pub fn set_cost(&mut self, cost: u128) {
        self.cost = cost;
    }

    pub fn get_cost(&self) -> u128 {
        self.cost
    }
}
fn add_running_modules(module: &RunningModuleWrapper) {
    unsafe {
        let v = match &mut RUNING_MODULES {
            Some(s) => s,
            None => {
                return;
            }
        };
        v.insert(module.index, module.clone());
    }
}

fn remove_running_modules(module: &RunningModuleWrapper, cost: u128, state: RunningState) {
    unsafe {
        let v = match &mut RUNING_MODULES {
            Some(s) => s,
            None => {
                return;
            }
        };

        match v.get_mut(&module.index) {
            Some(s) => {
                s.state = state;
                s.cost = cost;
            }
            None => {
                return;
            }
        }
    }
}

pub fn remove_dead_modules() {
    unsafe {
        let v = match &mut RUNING_MODULES {
            Some(s) => s,
            None => {
                return;
            }
        };

        let v2 = match &mut RUNING_MODULES {
            Some(s) => s,
            None => {
                return;
            }
        };
        for i in v {
            if i.1.get_state().eq(&RunningState::DEAD) {
                let i = i.0.clone();
                v2.remove(&i);
            }
        }
    }
}
pub fn get_running_modules() -> &'static Option<HashMap<u32, RunningModuleWrapper>> {
    unsafe { &RUNING_MODULES }
}
pub fn update_modules() {
    unsafe {
        let len = MODULES.len();
        for i in 0..len {
            if MODULES[i].is_change() {
                info!("{:?} file has changed", MODULES[i].metadata());
                let _ = MODULES[i].update();
            }
        }
    }
}

pub fn initialize_modules(dir: &str) -> &Vec<Box<dyn IActive + Sync>> {
    //Add Rust module in here
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let s = path.unwrap().path().to_str().unwrap().to_string();
        if s.ends_with(".rb") {
            let module = Box::new(RBModule::new(&s).unwrap());
            unsafe {
                match module.metadata() {
                    Some(s) => {
                        GLOB_POCS.push(s.clone());
                    }
                    None => {}
                };
            }
            unsafe {
                MODULES.push(module);
            }
        }
    }

    unsafe { &MODULES }
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
pub fn scaner_thread() -> JoinHandle<()> {
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
    rb_http_thread();
    let t = spawn(|| {
        let _ = rb_init();
        let modules = initialize_modules("./active/");
        // Thread::new(|| {
        //     loop {
        //         let command = get_command();
        //         if command.eq("reload") {
        //             let _ = update_and_get_modules("./active/");
        //         }
        //     }
        //     Fixnum::new(0)
        // });
        unsafe {
            if RUNING_MODULES.is_none() {
                RUNING_MODULES = Some(HashMap::new());
            }
        }
        let mut i = 0;
        loop {
            let will_run_modules = get_will_run_pocs();
            let index = match get_next_to_scan() {
                Some(s) => s,
                None => {
                    let _ = eval!("sleep(1)");
                    continue;
                }
            };
            unsafe {
                if WILL_RELOAD == true {
                    update_modules();
                    unset_reload();
                }
            }
            let mut s = vec![];
            for module in modules {
                let meta = match module.metadata() {
                    Some(o) => o,
                    None => {
                        continue;
                    }
                };
                if will_run_modules.contains(&meta.get_name().to_string()) == false {
                    continue;
                }

                
                if meta.get_type().eq(&ModuleType::RubyModule) {
                    let thread = Thread::new(|| {
                        let running_module = RunningModuleWrapper::new(&meta.get_name(), i, index);
                        
                        add_running_modules(&running_module);
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        let t1 = since_the_epoch.as_millis();
                        let v = module.passive_run(index);
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        let t2 = since_the_epoch.as_millis();
                        let t = t2 - t1;
                        info!("{} cost time: {} ms", meta.get_name(), t);
                        let state = match &v {
                            Ok(o) => RunningState::DEAD,
                            Err(e) => RunningState::EXCEPTION,
                        };
                        remove_running_modules(&running_module, t, state);
                        match v {
                            Ok(o) => {}
                            Err(e) => {
                                error!("{}", e);
                            }
                        }
                        Fixnum::new(0)
                    });
                    match thread.protect_send("run", &[]) {
                        Ok(o) => {}
                        Err(e) => {
                            error!("{}", e);
                        }
                    }

                    s.push(thread);
                } else if meta.get_type().eq(&ModuleType::RustModule) {
                    std::thread::spawn(move || {
                        let running_module = RunningModuleWrapper::new(&meta.get_name(), i, index);
                        
                        add_running_modules(&running_module);
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        let t1 = since_the_epoch.as_millis();
                        let v = module.passive_run(index);
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        let t2 = since_the_epoch.as_millis();
                        let t = t2 - t1;
                        info!("{} cost time: {} ms", meta.get_name(), t);
                        let state = match &v {
                            Ok(o) => RunningState::DEAD,
                            Err(e) => RunningState::EXCEPTION,
                        };
                        remove_running_modules(&running_module, t, state);
                        match v {
                            Ok(o) => {}
                            Err(e) => {
                                error!("{}", e);
                            }
                        }
                    });
                }
                i += 1;
            }

            // for thread in s {
            //     let _ = thread.protect_send("join", &[]);
            // }
        }
    });
    t
}

use std::{
    collections::HashMap, ptr::{addr_of, addr_of_mut}, sync::{
        mpsc,
        Arc, Mutex,
    }, thread::{spawn, JoinHandle}, time::{self, SystemTime, UNIX_EPOCH}
};

use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use log::{error, info};
//use rutie::{eval, Fixnum, Object, Thread};

use crate::{
    cmd::handlers::{SCAN_RECEIVER, SCAN_SENDER},
    modules::{
        remotescan::{information::{dirscan::DirScan, test::TestScan}, fuzz::unauth_bypass::UnauthBypass},
        get_next_to_scan, get_will_run_mods, IActive, ModuleType, GLOB_MODS, Task,
    },
    st_error,
    utils::STError,
};

pub static mut MODULE_INDEX: usize = 0;
pub static mut MODULES: Vec<Box<dyn IActive + Sync>> = Vec::new();
static mut RUNING_MODULES: Option<HashMap<i32, RunningModuleWrapper>> = None;
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
    index: i32,
    state: RunningState,
    args: String,
    cost: u128,
}

impl RunningModuleWrapper {
    pub fn new(name: &str, args: &str) -> Self {
        RunningModuleWrapper {
            name: name.to_string(),
            date: Local::now(),
            index: -1,
            state: RunningState::RUNNING,
            args: args.to_string(),
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
            return "DONE".to_string().red();
        } else if self.state.eq(&RunningState::EXCEPTION) {
            return "EXCEPTION".to_string().yellow();
        }

        "".to_string().red()
    }

    pub fn get_args(&self) -> &String {
        &self.args
    }

    pub fn set_cost(&mut self, cost: u128) {
        self.cost = cost;
    }

    pub fn get_cost(&self) -> u128 {
        self.cost
    }
}

static mut INDEX_OF_RUNNING_MODULE: i32 = 0;
static META_LOCKER: Mutex<i32> = Mutex::new(0);
pub fn add_running_modules(module: &mut RunningModuleWrapper) {
    unsafe {
        let _unused = META_LOCKER.lock();
        let v = match &mut *addr_of_mut!(RUNING_MODULES) {
            Some(s) => s,
            None => {
                return;
            }
        };
        v.insert(INDEX_OF_RUNNING_MODULE, module.clone());
        module.index = INDEX_OF_RUNNING_MODULE;
        INDEX_OF_RUNNING_MODULE += 1;
    }
}

pub fn remove_running_modules(module: &RunningModuleWrapper, cost: u128, state: RunningState) {
    unsafe {
        let _unused = META_LOCKER.lock();
        let v = match &mut *addr_of_mut!(RUNING_MODULES) {
            Some(s) => s,
            None => {
                return;
            }
        };

        if let Some(s) =  v.get_mut(&module.index) {
                s.state = state;
                s.cost = cost;
        }
    }
}

pub fn remove_dead_modules() {
    unsafe {
        let v = match &mut *addr_of_mut!(RUNING_MODULES) {
            Some(s) => s,
            None => {
                return;
            }
        };

        let v2 = match &mut *addr_of_mut!(RUNING_MODULES) {
            Some(s) => s,
            None => {
                return;
            }
        };
        for i in v {
            if i.1.get_state().eq(&RunningState::DEAD) {
                let i = *i.0;
                v2.remove(&i);
            }
        }
    }
}
pub fn get_running_modules() -> &'static Option<HashMap<i32, RunningModuleWrapper>> {
    unsafe { &*addr_of!(RUNING_MODULES) }
}
pub fn update_modules() {
    unsafe {
        for i in MODULES.iter_mut() {
            if i.is_change() {
                info!("{:?} file has changed", i.metadata());
                let _ = i.update();
            }
        }
    }
}

pub fn get_modules() -> &'static Vec<Box<dyn IActive + Sync>> {
    unsafe {
        &*addr_of!(MODULES)
    }
}
pub fn initialize_modules(dir: &str) -> &Vec<Box<dyn IActive + Sync>> {
    #[macro_export]
    macro_rules! add_module {
        (  $x:ident  ) => {
            unsafe {
                let module = Box::new($x::new());
                match module.metadata() {
                    Some(s) => {
                        GLOB_MODS.push(s.clone());
                    }
                    None => {}
                };
                MODULES.push(module);
            }
        };
    }
    //Add Rust module in here
    add_module!(DirScan);
    add_module!(UnauthBypass);
    add_module!(TestScan);
    unsafe { &*addr_of!(MODULES) }
}

pub static mut RUBY_COMMAND_SENDER: Option<std::sync::mpsc::Sender<String>> = None;
pub static mut RUBY_COMMAND_RECEIVER: Option<std::sync::mpsc::Receiver<String>> = None;
pub fn get_command() -> String {
    unsafe {
        let receiver = match &*addr_of!(RUBY_COMMAND_RECEIVER) {
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
        command.clone()
    }
}

pub fn send_command(command: &str) -> Result<(), STError> {
    unsafe {
        let sender = match &*addr_of!(RUBY_COMMAND_SENDER) {
            Some(s) => s,
            None => {
                return Err(STError::new("Sender is not available"));
            }
        };
        match sender.send(command.to_string()) {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(st_error!(e))
            }
        }
    }
}


pub fn scaner_thread() -> JoinHandle<()> {
    unsafe {
        if SCAN_SENDER.is_none() {
            let (tx, rx) = mpsc::channel::<Task>();
            SCAN_SENDER = Some(tx);
            SCAN_RECEIVER = Some(rx);
        }

    }

    let t = spawn(|| {
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
        let counter = Arc::new(Mutex::new(0));
        loop {
            let will_run_modules = get_will_run_mods();
            let task = match get_next_to_scan() {
                Some(s) => s,
                None => {
                    //In case cost too much CPU
                    std::thread::sleep(time::Duration::from_millis(100));
                    continue;
                }
            };
            let index = task.get_index();
            if task.is_once() {
                for module in modules {
                    let meta = match module.metadata() {
                        Some(o) => o,
                        None => {
                            continue;
                        }
                    };
                    if !meta.get_name().eq(task.get_mod_name()) {
                        continue;
                    }

                }
                continue;
            }
            unsafe {
                if WILL_RELOAD {
                    update_modules();
                    unset_reload();
                }
            }

            for module in modules {
                let meta = match module.metadata() {
                    Some(o) => o,
                    None => {
                        continue;
                    }
                };
                if !will_run_modules.contains(meta) {
                    
                    continue;
                }
                i += 1;
                if meta.get_type().eq(&ModuleType::RubyModule) {
                    
                } else if meta.get_type().eq(&ModuleType::RustModule) {
                    std::thread::spawn(move || {
                        let mut running_module = RunningModuleWrapper::new(meta.get_name(), &index.to_string());
                        add_running_modules(&mut running_module);
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
            }

            // for thread in s {
            //     let _ = thread.protect_send("join", &[]);
            // }
        }
    });
    t
}

use std::ptr::{addr_of, addr_of_mut};

use colored::Colorize;

use crate::{utils::STError, cmd::{handlers::{Log, CatResponse, ClearScreen
    , CatRequest, DebugLogInfo, DebugLevel, Sitemap, GetRequest, Scan, Test, SearchLog, Filter}, 
    issue_handler::{InfoIssue, ListIssues}, poc_handler::{PushMod, ListMods, LoadedMods, Reload, RunningMods, RemoveMod}, target_handler::{Push, ListTarget, ActiveScan, LocalScan}}};

use super::handlers::{Exit, Helper, ListHistory, ProxyLogInfo};
static mut CMD_HANDLER: CMDHandler = CMDHandler::new();

#[derive(Default)]
pub struct CMDHandler {
    procs: Vec<Box<dyn CMDProc>>,
}

#[derive(Default)]
pub struct  CMDOptions {
    auto_complete   : Vec<String>
}

#[allow(clippy::ptr_arg)]
pub trait CMDProc {
    fn get_name(&self) -> &str;

    fn get_opts(&self) -> &CMDOptions;

    fn process(&self, line: &Vec<&str>) -> Result<(), STError>;

    fn get_detail(&self) -> String;

    fn get_help(&self) -> String;
}

impl CMDHandler {
    pub const fn new() -> Self {
        
        Self { procs: Vec::new() }
    }

    pub fn process(&self, line: String) {
        let opts = line
            .split(' ')
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>();
        if opts.is_empty() {
            return ;
        }
        let proc_name = opts[0];
        for _proc in &self.procs {
            if _proc.get_name().eq(proc_name) {
                let res = _proc.process(&opts);
                match res {
                    Ok(o) => {

                    },
                    Err(e) => {
                        println!("[{}]:{}","-".red(), e);
                    }
                }
                return;
            }
        }

        println!("{} not found command, please use {} command for more commands", proc_name.green(), "help".green());
    }

    pub fn init(&mut self) {
        #[macro_export]
        macro_rules! hi {
            (  $x:ident  ) => {{
                self.procs.push(Box::new($x::new()))
            }};
        }
        hi!(Helper);
        hi!(ProxyLogInfo);
        hi!(ListHistory);
        hi!(Exit);
        hi!(Log);
        hi!(CatResponse);
        hi!(ClearScreen);
        hi!(CatRequest);
        hi!(DebugLogInfo);
        hi!(DebugLevel);
        hi!(Sitemap);
        hi!(GetRequest);
        hi!(Push);
        hi!(Scan);
        hi!(ListIssues);
        hi!(Test);
        hi!(PushMod);
        hi!(ListMods);
        hi!(LoadedMods);
        hi!(InfoIssue);
        hi!(ListTarget);
        hi!(Reload);
        hi!(RunningMods);
        hi!(RemoveMod);
        hi!(ActiveScan);
        hi!(SearchLog);
        hi!(LocalScan);
        hi!(Filter);
    }

    pub fn get_opts(&self) -> &Vec<String> {
        unimplemented!()
    }

    pub fn get_procs(&self) -> &Vec<Box<dyn CMDProc>> {
        &self.procs
    }

    pub fn get_handler() -> &'static CMDHandler {
        unsafe { &*addr_of!(CMD_HANDLER) }
    }

    pub fn get_handler_mut() -> &'static mut CMDHandler {
        unsafe { &mut *addr_of_mut!(CMD_HANDLER) }
    }
}

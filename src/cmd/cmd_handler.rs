use std::{collections::HashMap};

use crate::{utils::STError, cmd::handlers::{DebugLog, CatResponse, ClearScreen, CatRequest, DebugLogInfo, DebugLevel, Sitemap}};

use super::handlers::{Exit, Helper, ListHistory, ProxyLogInfo};
static mut CMD_HANDLER: CMDHandler = CMDHandler::new();

#[derive(Default)]
pub struct CMDHandler {
    procs: Vec<Box<dyn CMDProc>>,
}

pub type CMDOptions = HashMap<String, Option<String>>;

pub trait CMDProc {
    fn get_name(&self) -> &str;

    fn get_opts(&self) -> &CMDOptions;

    fn process(&self, line: &Vec<&str>) -> Result<(), STError>;
}

impl CMDHandler {
    pub const fn new() -> Self {
        let s = Self { procs: Vec::new() };
        s
    }

    pub fn process(&self, line: String) {
        let opts = line
            .split(" ")
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>();
        if opts.len() == 0 {
            return ;
        }
        let proc_name = opts[0];
        for _proc in &self.procs {
            if _proc.get_name().eq(proc_name) {
                let res = _proc.process(&opts);
            }
        }
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
        hi!(DebugLog);
        hi!(CatResponse);
        hi!(ClearScreen);
        hi!(CatRequest);
        hi!(DebugLogInfo);
        hi!(DebugLevel);
        hi!(Sitemap);
    }

    pub fn get_opts(&self) -> &Vec<String> {
        unimplemented!()
    }

    pub fn get_procs(&self) -> &Vec<Box<dyn CMDProc>> {
        &self.procs
    }

    pub fn get_handler() -> &'static CMDHandler {
        unsafe { &CMD_HANDLER }
    }

    pub fn get_handler_mut() -> &'static mut CMDHandler {
        unsafe { &mut CMD_HANDLER }
    }
}

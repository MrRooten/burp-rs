

use crate::utils::STError;
static mut CMD_HANDLER: CMDHandler = CMDHandler::new();


#[derive(Default)]
pub struct CMDHandler {
    proc_names       : Vec<String>,
}

impl CMDHandler {
    pub const fn new() -> Self {
        let mut s = Self { proc_names: Vec::new() };
        s
    }

    pub fn process(&self, line: String) {

    }

    pub fn init(&mut self) {
        self.proc_names.push("help".to_string());
        self.proc_names.push("help_info".to_string());
    }

    pub fn help(&mut self) -> Result<(),STError>{
        Ok(())
    }

    pub fn get_procs(&self) -> &Vec<String> {
        &self.proc_names
    }

    pub fn get_handler() -> &'static CMDHandler {
        unsafe {
            &CMD_HANDLER
        }
    }

    pub fn get_handler_mut() -> &'static mut CMDHandler {
        unsafe {
            &mut CMD_HANDLER
        }
    }
}
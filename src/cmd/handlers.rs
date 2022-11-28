use super::cmd_handler::*;

#[derive(Default)]
pub struct Helper {
    name    : String,
    opts    : CMDOptions,
    
}

impl CMDProc for Helper {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_opts(&self) -> &CMDOptions {
        return &self.opts;
    }

    fn process(&self, line: &Vec<&str>) -> Result<(),crate::utils::STError> {
        println!("help");
        Ok(())
    }
}

impl Helper {
    pub fn new() -> Self {
        Self {
            name: "help".to_string(),
            opts: Default::default()
        }
    }
}
use crate::modules::{IActive, Helper};

pub struct TestScan {

}

impl IActive for TestScan {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let result = Vec::default();
        println!("passive_run...");
        return Ok(result);
    }

    fn active_run(&self, url: &str, args: crate::modules::Args) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let result = Vec::default();
        println!("passive_run...");
        return Ok(result);
    }

    fn name(&self) -> String {
        "test".to_string()
    }

    fn help(&self) -> crate::modules::Helper {
        Helper::default()
    }
}
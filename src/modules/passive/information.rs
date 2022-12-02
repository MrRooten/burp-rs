use log::info;

use crate::modules::IPassive;


pub struct FingerIdentity;

impl IPassive for FingerIdentity {
    fn run(&self, index: u32) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let s = Vec::default();
        //println!("finger test");
        info!("finger test");
        return Ok(s);
    }

    fn name(&self) -> String {
        return "FingerIdentity".to_string();
    }

    fn help(&self) -> crate::modules::Helper {
        todo!()
    }
}
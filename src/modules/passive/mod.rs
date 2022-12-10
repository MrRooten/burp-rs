
pub mod finger_identity;
use self::finger_identity::CookieMatch;

use super::IPassive;

pub struct PassiveScanner {
    modules     : Vec<Box<(dyn IPassive+'static)>>
}

impl PassiveScanner {
    pub fn new() -> Self {
        let mut ret: Vec<Box<(dyn IPassive + 'static)>> = Vec::default();
        ret.push(Box::new(CookieMatch));
        Self {
            modules : ret
        }
    }

    pub fn passive_scan(&self, index: u32) {
        for module in &self.modules {
            let result = module.run(index);
        }
    }
}
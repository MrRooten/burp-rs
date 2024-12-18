use minus::{ExitStrategy};
use std::{
    thread::spawn, 
};

use crate::{utils::STError, st_error};

pub(crate) fn pager(s: &str, p: minus::Pager) -> Result<(), STError> {
    p.set_exit_strategy(ExitStrategy::PagerQuit).unwrap();
    // Initialize the pager
    let changes = || {
        p.push_str(s).unwrap_or(());
        Result::<(), Box<dyn std::error::Error>>::Ok(())
    };

    let p = p.clone();
    let res1 = spawn(|| minus::dynamic_paging(p));
    let res2 = changes();
    match res1.join() {
        Ok(o) => {
            match o {
                Ok(o) => {},
                Err(e) => {
                    return Err(st_error!(e));
                }
            }
        },
        Err(e) => {
            
        }
    };
    match res2 {
        Ok(o) => {},
        Err(e) => {
            return Err(STError::new(&format!("{:?}",e)));
        },
    }

    Ok(())
}

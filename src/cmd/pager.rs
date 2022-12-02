use minus::{dynamic_paging, MinusError, Pager, page_all};
use std::{
    fmt::Write, 
    thread::{spawn, sleep}, 
    time::Duration
};

use crate::{st_error, utils::STError};

pub(crate) fn pager(s: &str) -> Result<(), STError> {
    // Initialize the pager
    let mut pager = Pager::new();
    pager.set_run_no_overflow(true)?;
    let e = pager.set_exit_strategy(minus::ExitStrategy::PagerQuit);
    match e {
        Ok(o) => {},
        Err(e) => {
            return Err(st_error!(e));
        }
    }
    match writeln!(pager,"{}", s) {
        Ok(o) => {},
        Err(e) => {
            return Err(st_error!(e));
        }
    }
    match page_all(pager) {
        Ok(o) => {},
        Err(e) => {
            return Err(st_error!(e));
        }
    };
    Ok(())
}

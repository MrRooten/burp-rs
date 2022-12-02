use minus::{dynamic_paging, MinusError, Pager, page_all};
use std::{
    fmt::Write, 
    thread::{spawn, sleep}, 
    time::Duration
};

pub(crate) fn pager(s: &str) -> Result<(), MinusError> {
    // Initialize the pager
    let mut pager = Pager::new();
    pager.set_run_no_overflow(true)?;
    pager.set_exit_strategy(minus::ExitStrategy::PagerQuit);
    for i in 0..=10u32 {
        writeln!(pager, "{}", i)?;
    }
    page_all(pager)?;
    Ok(())
}

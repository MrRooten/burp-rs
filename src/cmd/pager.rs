use minus::{dynamic_paging, MinusError, Pager};
use std::{
    fmt::Write, 
    thread::{spawn, sleep}, 
    time::Duration
};

pub(crate) fn pager(s: &str) -> Result<(), MinusError> {
    // Initialize the pager
    let mut pager = Pager::new();
    // Run the pager in a separate thread
    let pager2 = pager.clone();
    pager.set_exit_strategy(minus::ExitStrategy::PagerQuit).unwrap();
    let pager_thread = spawn(move || dynamic_paging(pager2));
    

    writeln!(pager, "{}", s);

    pager_thread.join().unwrap()?;
    Ok(())
}

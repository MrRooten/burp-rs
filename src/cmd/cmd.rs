use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::highlight::Highlighter;
use rustyline::hint::HistoryHinter;
use rustyline::{
    Cmd, ConditionalEventHandler, Editor, Event, EventContext, EventHandler, KeyEvent, RepeatCount,
    Result,
};
use rustyline_derive::{Completer, Helper, Hinter, Validator};

use super::cmd_handler::CMDHandler;

#[derive(Completer, Helper, Hinter, Validator)]
struct MyHelper(#[rustyline(Hinter)] HistoryHinter);

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Owned(format!("\x1b[1;32m{prompt}\x1b[m"))
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[1m{hint}\x1b[m"))
    }
}

#[derive(Clone)]
struct CompleteHintHandler;
impl ConditionalEventHandler for CompleteHintHandler {
    fn handle(&self, evt: &Event, _: RepeatCount, _: bool, ctx: &EventContext) -> Option<Cmd> {
        if !ctx.has_hint() {
            return None; // default
        }
        if let Some(k) = evt.get(0) {
            #[allow(clippy::if_same_then_else)]
            if *k == KeyEvent::ctrl('E') {
                Some(Cmd::CompleteHint)
            } else if *k == KeyEvent::alt('f') && ctx.line().len() == ctx.pos() {
                let text = ctx.hint_text()?;
                let mut start = 0;
                if let Some(first) = text.chars().next() {
                    if !first.is_alphanumeric() {
                        start = text.find(|c: char| c.is_alphanumeric()).unwrap_or_default();
                    }
                }

                let text = text
                    .chars()
                    .enumerate()
                    .take_while(|(i, c)| *i <= start || c.is_alphanumeric())
                    .map(|(_, c)| c)
                    .collect::<String>();

                Some(Cmd::Insert(1, text))
            } else {
                None
            }
        } else {
            unreachable!()
        }
    }
}

struct TabEventHandler;
impl ConditionalEventHandler for TabEventHandler {
    fn handle(&self, evt: &Event, n: RepeatCount, _: bool, ctx: &EventContext) -> Option<Cmd> {
        debug_assert_eq!(*evt, Event::from(KeyEvent::from('\t')));
        let handler = CMDHandler::get_handler();
        let procs = handler.get_procs();
        let mut will_show = Vec::<String>::default();
        for proc in procs {
            if proc.starts_with(ctx.line()) {
                will_show.push(proc.to_string());
            }
        }

        if will_show.len() == 1 {
            let ret = will_show[0][ctx.pos()..].to_string() + " ";
            return Some(Cmd::Insert(n, ret));
        }

        if will_show.len() > 0 {
            println!("{:?}",will_show);
        }
        Some(Cmd::Insert(n, "".to_string()))
    }
}

pub fn cmd() -> Result<()> {
    let handler = CMDHandler::get_handler_mut();
    handler.init();
    let mut rl = Editor::<MyHelper>::new()?;
    rl.set_helper(Some(MyHelper(HistoryHinter {})));

    let ceh = Box::new(CompleteHintHandler);
    rl.bind_sequence(KeyEvent::ctrl('E'), EventHandler::Conditional(ceh.clone()));
    rl.bind_sequence(KeyEvent::alt('f'), EventHandler::Conditional(ceh));
    rl.bind_sequence(
        KeyEvent::from('\t'),
        EventHandler::Conditional(Box::new(TabEventHandler)),
    );

    rl.bind_sequence(
        Event::KeySeq(vec![KeyEvent::ctrl('X'), KeyEvent::ctrl('E')]),
        EventHandler::Simple(Cmd::Suspend), // TODO external editor
    );

    loop {
        let line = rl.readline("> ")?;
        rl.add_history_entry(line.as_str());
        println!("Line: {line}");
    }
}
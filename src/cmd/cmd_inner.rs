use std::borrow::Cow::{self, Borrowed, Owned};

use colored::Colorize;
use rustyline::completion::{Completer};

use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent, ExternalPrinter};
use rustyline_derive::{Completer, Helper, Hinter, Validator};

use super::cmd_handler::CMDHandler;

struct MyCompleter;

impl Completer for MyCompleter {
    type Candidate = String;
    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let _ = (line, pos, ctx);
        let handler = CMDHandler::get_handler();
        let candiates = handler.get_procs();
        let mut res = Vec::<String>::default();
        for candidate in candiates {
            if candidate.get_name().starts_with(line) {
                res.push(candidate.get_name().to_string());
            }
        }
        Ok((0, res))
    }

    fn update(&self, line: &mut rustyline::line_buffer::LineBuffer, start: usize, elected: &str) {
        let end = line.pos();
        line.replace(start..end, elected);
    }

}


#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: MyCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

// To debug rustyline:
// RUST_LOG=rustyline=debug cargo run --example example 2> debug.log
pub fn cmd() -> rustyline::Result<()> {
    let handler = CMDHandler::get_handler_mut();
    handler.init();
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();
    let h = MyHelper {
        completer: MyCompleter,
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    rl.bind_sequence(KeyEvent::ctrl('l'), Cmd::ClearScreen);
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    //let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread", "vdso"]).build().unwrap();
    
    let mut count = 1;
    loop {
        // match guard.report().build() {
        //     Ok(report) => {
        //         let mut file = File::create("profile.pb").unwrap();
        //         let mut profile = report.pprof().unwrap();
        
        //         let mut content = Vec::new();
        //         profile.write_to_vec(&mut content).unwrap();
        //         file.write_all(&content).unwrap();
        
        //         //println!("report: {}", &report);
        //     }
        //     Err(_) => {}
        // };
        let p = format!("burp-rs {count}> ");
        rl.helper_mut().expect("No helper").colored_prompt = format!("{}",p.bright_blue());
        let readline = rl.readline(&p);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.eq("clear") {
                    let mut printer = rl.create_external_printer()?;
                    match printer.print("\x1B[2J\x1B[1;1H".to_string()) {
                        Ok(()) => {

                        },
                        Err(e) => {
                            println!("Can not print clear chars sequence");
                        }
                    }

                }

                if line.eq("shell") {
                    println!("Shell mode, not implement")
                }

                if line.eq("exit_shell") {
                    println!("Shell mode exit")
                }
                let handler = CMDHandler::get_handler();
                handler.process(line)
            }
            Err(ReadlineError::Interrupted) => {
                println!("Please use exit command to exit");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Encountered Eof");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
        count += 1;
    }
    rl.append_history("history.txt")
}
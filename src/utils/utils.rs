use std::default::Default;

use html5ever::tendril::*;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::Token::CommentToken;
use html5ever::tokenizer::{CharacterTokens, EndTag, NullCharacterToken, StartTag, TagToken};
use html5ever::tokenizer::{
    ParseError, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};


struct TokenPrinter {
    in_char_run: bool,
    in_script: bool,
    unparse_text: String,
    indent_num: u32,
    result_s    : String
}


impl TokenPrinter {
    fn is_char(&mut self, is_char: bool) {
        match (self.in_char_run, is_char) {
            (false, true) => {
                //print!("\"");
            }
            (true, false) => {
                //println!("\"");
            }
            _ => {
                self.in_char_run = is_char;
            }
        }
        self.in_char_run = is_char;
    }

    fn do_char(&mut self, c: char) {
        if self.in_script {
            self.unparse_text.push(c);
        } else {
            self.is_char(true);
            //print!("{}", c);
        }
    }

    fn process_indent(&mut self) {
        let mut i = 0;
        while i < self.indent_num {
            self.result_s.push_str("  ");
            i += 1;
        }
    }
}

impl TokenSink for TokenPrinter {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            CharacterTokens(b) => {
                for c in b.chars() {
                    self.do_char(c);
                }
            }
            NullCharacterToken => self.do_char(' '),
            TagToken(tag) => {
                self.is_char(false);
                if !tag.name.eq("script") && self.in_script==true {
                    return TokenSinkResult::Continue;
                }
                // This is not proper HTML serialization, of course.
                match tag.kind {
                    StartTag => {
                        self.process_indent();
                        self.indent_num += 1;
                        self.result_s.push_str(&format!("<\x1b[32m{}\x1b[0m", tag.name));
                        if tag.name.eq("script") {
                            self.in_script = true;
                        }
                    }
                    EndTag => {
                        if tag.name.eq("script") {
                            self.result_s.push_str(&format!("{}",self.unparse_text));
                            self.unparse_text = String::new();
                        }
                        if self.indent_num >= 1 {
                            self.indent_num -= 1;
                        }
                        self.process_indent();
                        self.result_s.push_str(&format!("<\x1b[31m/{}\x1b[0m", tag.name));
                        if tag.name.eq("script") {
                            self.in_script = false;
                        }
                    }
                }
                for attr in tag.attrs.iter() {
                    self.result_s.push_str(&format!(
                        " \x1b[36m{}\x1b[0m='\x1b[34m{}\x1b[0m'",
                        attr.name.local, attr.value
                    ));
                }
                if tag.self_closing {
                    self.result_s.push_str(&format!(" \x1b[31m/\x1b[0m"));
                    if self.indent_num >= 1 {
                        self.indent_num -= 1;
                    }
                }

                self.result_s.push_str(&format!(">\n"));
            }
            ParseError(err) => {
                if self.in_script == true {
                    return TokenSinkResult::Continue;
                }
                self.is_char(false);
                //println!("ERROR: {}", err);
            }
            CommentToken(comment) => {
                if self.in_script {
                    for c in comment.chars() {
                        self.unparse_text.push(c);
                    }
                }
            }
            _ => {
                if self.in_script == true {
                    return TokenSinkResult::Continue;
                }

                self.is_char(false);
                //println!("OTHER: {:?}", token);
            }
        }
        TokenSinkResult::Continue
    }
}

pub fn tidy_html(html: &str) -> String {
    let sink = TokenPrinter {
        in_char_run: false,
        in_script: false,
        unparse_text: "".to_string(),
        indent_num: 0,
        result_s    : "".to_string()
    };

    let mut tok = Tokenizer::new(
        sink,
        TokenizerOpts {
            profile: true,
            ..Default::default()
        },
    );
    let chunk: Tendril<fmt::Bytes> = html.as_bytes().try_into().unwrap();
    let mut html_buffer = BufferQueue::new();
    html_buffer.push_back(chunk.try_reinterpret().unwrap());
    let _ = tok.feed(&mut html_buffer);
    tok.sink.result_s.push_str("\0");
    tok.sink.result_s
}



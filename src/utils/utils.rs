use std::borrow::Cow;
use std::default::Default;
use std::io::BufRead;
use std::path::Path;

use html5ever::tendril::*;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::Token::CommentToken;
use html5ever::tokenizer::{CharacterTokens, EndTag, NullCharacterToken, StartTag, TagToken};
use html5ever::tokenizer::{
    ParseError, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};
use syntect::dumps::{dump_to_file, from_dump_file};
use syntect::easy::{HighlightFile, HighlightLines};
use syntect::highlighting::{Highlighter, Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

struct TokenPrinter {
    in_char_run: bool,
    in_script: bool,
    unparse_text: String,
    indent_num: u32,
    result_s: String,
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

    fn special_tag(&self, name: &str) -> bool {
        let tags = vec![
            "area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen",
            "link", "meta", "param", "source", "track", "wbr",
        ];
        if tags.contains(&name) {
            return true;
        }

        return false;
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
                if !tag.name.eq("script") && self.in_script == true {
                    return TokenSinkResult::Continue;
                }
                // This is not proper HTML serialization, of course.
                match tag.kind {
                    StartTag => {
                        self.process_indent();
                        self.indent_num += 1;
                        self.result_s
                            .push_str(&format!("<{}", tag.name));
                        if tag.name.eq("script") {
                            self.in_script = true;
                        }
                    }
                    EndTag => {
                        if tag.name.eq("script") {
                            let (pretty, _) = prettify_js::prettyprint(&self.unparse_text);
                            self.result_s.push_str(&format!("{}", pretty));
                            self.unparse_text = String::new();
                        }
                        if self.indent_num >= 1 {
                            self.indent_num -= 1;
                        }
                        self.process_indent();

                        self.result_s
                            .push_str(&format!("</{}", tag.name));
                        if tag.name.eq("script") {
                            self.in_script = false;
                        }
                    }
                }
                for attr in tag.attrs.iter() {
                    self.result_s.push_str(&format!(
                        " {}='{}'",
                        attr.name.local, attr.value
                    ));
                }
                let mut self_closing = tag.self_closing;
                if self.special_tag(&tag.name) {
                    self_closing = true;
                }
                if self_closing {
                    self.result_s.push_str(&format!(" /"));
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
        result_s: "".to_string(),
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

fn load_theme(tm_file: &str, enable_caching: bool) -> Theme {
    let tm_path = Path::new(tm_file);

    if enable_caching {
        let tm_cache = tm_path.with_extension("tmdump");

        if tm_cache.exists() {
            from_dump_file(tm_cache).unwrap()
        } else {
            let theme = ThemeSet::get_theme(tm_path).unwrap();
            dump_to_file(&theme, tm_cache).unwrap();
            theme
        }
    } else {
        ThemeSet::get_theme(tm_path).unwrap()
    }
}

pub fn highlighter(js: &str) -> String{
    let mut res = String::new();
    let theme_file: String = "base16-ocean.dark".to_string();
    let ts = ThemeSet::load_defaults();
    let theme = ts
        .themes
        .get(&theme_file)
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(load_theme(&theme_file, false)));

    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = ss.find_syntax_by_extension("js").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let s = js;
    for line in LinesWithEndings::from(s) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        res.push_str(&escaped);
    }
    return res;
}

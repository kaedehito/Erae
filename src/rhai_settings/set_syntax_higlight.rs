#![allow(unused)]

use syntect::easy::HighlightLines;
use syntect::highlighting::{self, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};


pub fn set_syntax_higlight(enable: bool) {}

pub fn syntax_return(content: &str, extension: &str) -> String {
    let ps = SyntaxSet::load_defaults_newlines(); // 標準のシンタックスセット
    let ts = ThemeSet::load_defaults(); // 標準のテーマセット

    let mut ret = vec![String::new()];

    if let Some(syntax) = ps.find_syntax_by_extension(extension) {
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);
        for line in LinesWithEndings::from(content) {
            let ranges: Vec<(highlighting::Style, &str)> = h.highlight_line(line, &ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            ret.push(escaped);
        }

        ret.join("\n")
    } else {
        return String::from(content);
    }
}

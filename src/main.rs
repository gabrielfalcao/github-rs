mod client;
mod config;
use client::Client;
use config::Config;
use std::io::{self, Write};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};


fn main() {
    let config = Config::load("/Users/gabrielfalcao/.config/github-rs/config.yml");
    let username = config.github_username();
    let client = Client::new(config);

    println!("Retrieving info about github user: {}", username);
    let data = client.get_user().unwrap();

    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);
    let s = data.as_str();
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        io::stdout().write_all(escaped.as_bytes()).unwrap();
    }
    println!("\x1b[0m");
}

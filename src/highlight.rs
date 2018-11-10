use errors::InspectError;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// Apply syntax highlighting to a string
pub fn highlight(input: String, theme: String) -> Result<String, InspectError> {
    // TODO: Load these once at the start of our program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes[&theme]);

    let mut formatted: String = String::new();
    for line in LinesWithEndings::from(&input) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        formatted.push_str(&escaped)
    }
    Ok(formatted)
}

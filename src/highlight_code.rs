use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub struct Highlighter {
    ss: SyntaxSet,
    theme: Theme,
}

impl Highlighter {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        let now = std::time::Instant::now();

        let ss = SyntaxSet::load_defaults_newlines();
        let theme = ThemeSet::load_defaults().themes["base16-eighties.dark"].to_owned();

        #[cfg(debug_assertions)]
        eprintln!(
            "load code highlighter... {:?}",
            std::time::Instant::now().duration_since(now)
        );

        Self { ss, theme }
    }

    pub fn highlight(&self, language: &str, text: &str) -> String {
        let syntax = self
            .ss
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.ss.find_syntax_plain_text());
        highlighted_html_for_string(text, &self.ss, syntax, &self.theme)
    }
}

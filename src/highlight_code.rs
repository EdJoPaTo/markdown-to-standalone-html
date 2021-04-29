use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub struct Highlighter {
    ss: SyntaxSet,
    ts: ThemeSet,
    theme_name: String,
}

impl Highlighter {
    pub fn new(theme_name: &str) -> Self {
        #[cfg(debug_assertions)]
        let now = std::time::Instant::now();

        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        #[cfg(debug_assertions)]
        eprintln!(
            "load code highlighter... {:?}",
            std::time::Instant::now().duration_since(now)
        );

        Self {
            ss,
            ts,
            theme_name: theme_name.to_owned(),
        }
    }

    pub fn highlight(&self, language: &str, text: &str) -> String {
        let theme = &self.ts.themes[&self.theme_name];
        let syntax = self
            .ss
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.ss.find_syntax_plain_text());
        highlighted_html_for_string(text, &self.ss, syntax, theme)
    }
}

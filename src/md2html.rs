use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag};

use crate::heading::{Heading, Headings};
use crate::highlight_code::Highlighter;

pub fn parse(markdown: &str) -> (String, Vec<Heading>) {
    let parser = Parser::new(markdown);

    let mut heading_level = 0;
    let mut headings = Headings::new();

    let code_highlighter = Highlighter::new();
    let mut code_language = None;

    let parser = parser.filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            code_language = Some(lang.to_string());
            Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))))
        }
        Event::Start(Tag::Heading(level @ 1..=6)) => {
            heading_level = level;
            None
        }
        Event::Text(text) if heading_level > 0 => {
            let anchor = headings.create_from_title(heading_level, &text);
            let event = Event::Html(CowStr::from(format!(
                "<h{} id=\"{}\">{}",
                heading_level, anchor, text
            )));
            heading_level = 0;
            Some(event)
        }
        Event::Text(text) if code_language.is_some() => {
            let language = &code_language.as_ref().unwrap();
            let html = code_highlighter.highlight(language, &text);
            code_language = None;
            Some(Event::Html(CowStr::from(html)))
        }
        _ => Some(event),
    });

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);
    (html_buf, headings.list)
}

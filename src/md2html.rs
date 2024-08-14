use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag, TagEnd};

use crate::heading::{Heading, Headings};
use crate::highlight_code::Highlighter;

pub fn parse(markdown: &str) -> (String, Vec<Heading>) {
    let parser = Parser::new(markdown);

    let mut heading_level = None;
    let mut heading_texts = Vec::new();
    let mut headings = Headings::default();

    let code_highlighter = Highlighter::new();
    let mut code_language = None;

    let parser = parser.filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            code_language = Some(lang.to_string());
            Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))))
        }
        Event::Start(Tag::Heading { level, .. }) => {
            heading_level = Some(level);
            heading_texts = Vec::new();
            None
        }
        Event::End(TagEnd::Heading(level)) => {
            assert_eq!(
                heading_level,
                Some(level),
                "the heading level should start and end the same"
            );
            let text = heading_texts.join("");
            let anchor = headings.add(level, &text);
            let event = Event::Html(CowStr::from(format!(
                "<{level} id=\"{anchor}\">{text}</{level}>"
            )));
            heading_level = None;
            heading_texts = Vec::new();
            Some(event)
        }
        Event::Text(text) if heading_level.is_some() => {
            heading_texts.push(text.to_string());
            None
        }
        Event::Text(text) if code_language.is_some() => {
            let language = code_language.as_ref().unwrap();
            let event = match code_highlighter.highlight(language, &text) {
                Ok(html) => Event::Html(CowStr::from(html)),
                Err(err) => {
                    eprintln!("Warning: Failed creating source code formatting: {err}");
                    Event::Text(text)
                }
            };
            code_language = None;
            Some(event)
        }
        _ => Some(event),
    });

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);
    (html_buf, headings.finish())
}

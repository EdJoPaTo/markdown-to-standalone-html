use pulldown_cmark::{CowStr, Event, Parser, Tag};
use regex::Regex;

use crate::heading::Heading;

pub fn parse(markdown: &str) -> (String, Vec<Heading>) {
    let parser = Parser::new(markdown);

    let mut heading_anchors = Vec::new();
    let mut heading_level = 0;
    let mut headings = Vec::new();

    let parser = parser.filter_map(|event| match event {
        Event::Start(Tag::Heading(level @ 1..=6)) => {
            heading_level = level;
            None
        }
        Event::Text(text) => {
            if heading_level == 0 {
                Some(Event::Text(text))
            } else {
                let anchor = create_anchor_of_title(&mut heading_anchors, &text);
                let event = Event::Html(CowStr::from(format!(
                    "<h{} id=\"{}\">{}",
                    heading_level, anchor, text
                )));
                headings.push(Heading {
                    level: heading_level,
                    anchor,
                    title: text.to_string(),
                });
                heading_level = 0;
                Some(event)
            }
        }
        _ => Some(event),
    });

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);
    (html_buf, headings)
}

fn create_anchor_of_title(existing: &mut Vec<String>, title: &str) -> String {
    let re = Regex::new("[^a-zA-Z\\d]+").unwrap();
    let main = re
        .replace_all(title, "-")
        .trim_matches('-')
        .to_ascii_lowercase();

    let mut anchor = main.to_owned();
    let mut index = 1;
    while existing.contains(&anchor) {
        index += 1;
        anchor = format!("{}-{}", main, index);
    }
    existing.push(anchor.to_owned());
    anchor
}

#[test]
fn anchor_of_title_examples() {
    let mut db = vec![];
    assert_eq!("a-b", create_anchor_of_title(&mut db, " A b"));
    assert_eq!(
        "passw-rter",
        create_anchor_of_title(&mut db, "passw\u{f6}rter")
    );
}

#[test]
fn anchor_of_title_is_unique() {
    let mut db = vec![];
    assert_eq!("a", create_anchor_of_title(&mut db, "a"));
    assert_eq!("a-2", create_anchor_of_title(&mut db, "a"));
    assert_eq!("a-3", create_anchor_of_title(&mut db, "a"));
}

use std::fs;
use std::io::Read;

use handlebars::Handlebars;
use serde_json::json;

mod cli;
mod heading;
mod highlight_code;
mod md2html;

const GENERATOR: &str = concat!(env!("CARGO_PKG_REPOSITORY"), " ", env!("CARGO_PKG_VERSION"));
const TEMPLATE: &str = include_str!("template.html");

fn main() {
    let matches = cli::build().get_matches();

    if matches.subcommand_matches("template").is_some() {
        println!("{}", TEMPLATE);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("raw") {
        let input_path = matches.value_of("markdown-file").unwrap();
        let markdown = read_markdown(input_path);
        let (html_part, _) = md2html::parse(&markdown);
        println!("{}", html_part);
        return;
    }

    let input_path = matches.value_of("markdown-file").unwrap();
    let markdown = read_markdown(input_path);
    let (html_part, headings) = md2html::parse(&markdown);

    let template = matches.value_of("template-file").map_or_else(
        || TEMPLATE.to_string(),
        |path| fs::read_to_string(path).expect("failed to read template file"),
    );

    let toc_part = heading::to_html_toc(&headings);

    let title = headings.first().map(|o| o.title.clone());

    let body = format!(
        r#"<nav class="toc">{}</nav><main>{}</main>"#,
        toc_part, html_part
    );

    let result_html = Handlebars::new()
        .render_template(
            &template,
            &json!({
                "body": body,
                "generator": GENERATOR,
                "title": title,
            }),
        )
        .expect("failed to render template");

    println!("{}", result_html);
}

fn read_markdown(input_path: &str) -> String {
    if input_path == "-" {
        let mut input = String::new();
        std::io::stdin()
            .lock()
            .read_to_string(&mut input)
            .expect("failed to read from stdin");
        input
    } else {
        fs::read_to_string(input_path).expect("failed to read markdown file")
    }
}

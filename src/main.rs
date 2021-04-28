use std::fs;
use std::io::Read;

use handlebars::Handlebars;
use serde_json::json;

mod cli;
mod heading;

const GENERATOR: &str = concat!(env!("CARGO_PKG_REPOSITORY"), " ", env!("CARGO_PKG_VERSION"));
const TEMPLATE: &str = include_str!("template.html");

fn main() {
    let matches = cli::build().get_matches();

    if matches.subcommand_matches("template").is_some() {
        println!("{}", TEMPLATE);
        return;
    }

    let input_path = matches.value_of("markdown-file").unwrap();
    let markdown = if input_path == "-" {
        let mut input = String::new();
        std::io::stdin()
            .lock()
            .read_to_string(&mut input)
            .expect("failed to read from stdin");
        input
    } else {
        fs::read_to_string(input_path).expect("failed to read markdown file")
    };

    let template = matches.value_of("template-file").map_or_else(
        || TEMPLATE.to_string(),
        |path| fs::read_to_string(path).expect("failed to read template file"),
    );

    let html_part = markdown::to_html(&markdown);

    let headings = heading::from_html(&html_part);
    let toc_part = heading::to_html_toc(&headings);

    let title = headings.first().map(|o| o.title.to_owned());

    let body = format!(
        r#"<div class="toc">{}</div><main>{}</main>"#,
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

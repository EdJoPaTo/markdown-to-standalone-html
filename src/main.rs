use std::env;
use std::fs;

mod heading;

const TEMPLATE: &str = include_str!("template.html");

fn main() {
    let path = env::args()
        .nth(1)
        .expect("failed to read command line argument");
    let markdown = fs::read_to_string(path).expect("failed to read file from argument");

    let html_part = markdown::to_html(&markdown);

    let headings = heading::from_html(&html_part);
    let toc_part = heading::to_html_toc(&headings);

    let result_html = TEMPLATE.replace(
        "<main></main>",
        &format!(
            r#"<div class="toc">{}</div><main>{}</main>"#,
            toc_part, html_part
        ),
    );
    println!("{}", result_html);
}

use std::fs;
use std::io::Read;

mod cli;
mod heading;

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

    let result_html = template.replace(
        "<main></main>",
        &format!(
            r#"<div class="toc">{}</div><main>{}</main>"#,
            toc_part, html_part
        ),
    );
    println!("{}", result_html);
}

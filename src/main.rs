use std::fs;
use std::io::Read;
use std::path::Path;

use clap::Parser;
use handlebars::Handlebars;
use serde_json::json;

use crate::inline_assets::inline_assets;

mod cli;
mod heading;
mod highlight_code;
mod inline_assets;
mod md2html;

const GENERATOR: &str = concat!(env!("CARGO_PKG_REPOSITORY"), " ", env!("CARGO_PKG_VERSION"));
const TEMPLATE: &str = include_str!("template.html");

fn main() {
    let matches = cli::Cli::parse();

    match matches.subcommands {
        cli::SubCommands::Template {} => {
            println!("{TEMPLATE}");
        }
        cli::SubCommands::Raw { markdown_file } => {
            let markdown = read_markdown(&markdown_file);
            let (html_part, _) = md2html::parse(&markdown);
            println!("{html_part}");
        }
        cli::SubCommands::Render {
            template_file,
            no_inline,
            markdown_file,
        } => {
            let markdown = read_markdown(&markdown_file);
            let (html_part, headings) = md2html::parse(&markdown);

            let template = template_file.map_or_else(
                || TEMPLATE.to_string(),
                |path| fs::read_to_string(path).expect("failed to read template file"),
            );

            let toc_part = heading::to_html_toc(&headings);

            let title = headings.first().map(|o| o.title.clone());

            let body = format!(r#"<nav class="toc">{toc_part}</nav><main>{html_part}</main>"#);

            let rendered = Handlebars::new()
                .render_template(
                    &template,
                    &json!({
                        "body": body,
                        "generator": GENERATOR,
                        "title": title,
                    }),
                )
                .expect("failed to render template");

            if no_inline {
                println!("{rendered}");
            } else {
                let inlined = match inline_assets(rendered.clone()) {
                    Ok(html) => html,
                    Err(err) => {
                        eprintln!(
                            "INFO: html assets are not inlined. Is monolith installed and in PATH? Reason: {err}"
                        );
                        rendered
                    }
                };

                println!("{inlined}");
            }
        }
    }
}

fn read_markdown(input_path: &Path) -> String {
    if input_path.to_str() == Some("-") {
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

use clap::{command, Arg, Command, ValueHint};

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn build() -> Command<'static> {
    let markdown_file = Arg::new("markdown-file")
        .value_hint(ValueHint::FilePath)
        .value_name("FILE")
        .takes_value(true)
        .required(true)
        .help("Markdown file to be parsed. Use - to read from stdin instead.");

    command!()
        .name("Markdown to Standalone HTML")
        .subcommand_negates_reqs(true)
        .subcommand(
            Command::new("template").about("Print the included template to stdout."),
        )
        .subcommand(
            Command::new("raw")
                .about("Only parse the markdown to html without any further modifications")
                .arg(&markdown_file),
        )
        .arg(
            Arg::new("template-file")
                .long("template")
                .short('t')
                .env("TEMPLATE_FILE")
                .value_hint(ValueHint::FilePath)
                .value_name("FILE")
                .takes_value(true)
                .help("Template file to be used instead of the builtin one.")
                .long_help("Template file to be used instead of the builtin one. Use the subcommand template to print the builtin template to stdout."),
        )
        .arg(
            Arg::new("no-inline")
                .long("no-inline")
                .short('i')
                .env("NO_INLINE")
                .help("Don't try to inline assets.")
                .long_help("Don't try to inline assets. Normally assets are inlined with monolith. When monolith is not in PATH a warning is shown. This warning is also suppressed with this flag."),
        )
        .arg(
            &markdown_file,
        )
}

#[test]
fn verify() {
    build().debug_assert();
}

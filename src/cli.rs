use clap::{app_from_crate, App, AppSettings, Arg};

#[must_use]
pub fn build() -> App<'static> {
    let markdown_file = Arg::new("markdown-file")
        .value_name("FILE")
        .takes_value(true)
        .required(true)
        .help("Markdown file to be parsed. Use - to read from stdin instead.");

    app_from_crate!()
        .name("Markdown to Standalone HTML")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandsNegateReqs)
        .subcommand(
            App::new("template").about("Print the included template to stdout."),
        )
        .subcommand(
            App::new("raw")
                .about("Only parse the markdown to html without any further modifications")
                .arg(&markdown_file),
        )
        .arg(
            Arg::new("template-file")
                .long("template")
                .short('t')
                .value_name("FILE")
                .takes_value(true)
                .help("Template file to be used instead of the builtin one.")
                .long_help("Template file to be used instead of the builtin one. Use the subcommand template to print the builtin template to stdout."),
        )
        .arg(
            Arg::new("no-inline")
                .long("no-inline")
                .short('i')
                .help("Don't try to inline assets.")
                .long_help("Don't try to inline assets. Normally assets are inlined with monolith. When monolith is not in PATH a warning is shown. This warning is also suppressed with this flag."),
        )
        .arg(
            &markdown_file,
        )
}

#[test]
fn verify_app() {
    build().debug_assert();
}

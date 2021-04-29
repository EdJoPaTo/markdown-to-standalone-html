use clap::{App, AppSettings, Arg, SubCommand};

pub fn build() -> App<'static, 'static> {
    let markdown_file = Arg::with_name("markdown-file")
        .value_name("FILE")
        .takes_value(true)
        .required(true)
        .help("Markdown file to be parsed. Use - to read from stdin instead.");

    App::new("Markdown to Standalone HTML")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandsNegateReqs)
        .subcommand(
            SubCommand::with_name("template").about("Print the included template to stdout."),
        )
        .subcommand(
            SubCommand::with_name("raw")
                .about("Only parse the markdown to html without any further modifications")
                .arg(&markdown_file),
        )
        .arg(
            Arg::with_name("template-file")
                .long("template")
                .short("t")
                .value_name("FILE")
                .takes_value(true)
                .help("Template file to be used instead of the builtin one.")
                .long_help("Template file to be used instead of the builtin one. Use the subcommand template to print the builtin template to stdout."),
        )
        .arg(
            &markdown_file,
        )
}

use clap::{App, AppSettings, Arg, SubCommand};

pub fn build() -> App<'static, 'static> {
    App::new("Markdown to Standalone HTML")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandsNegateReqs)
        .subcommand(
            SubCommand::with_name("template").about("Print the included template to stdout."),
        )
        .arg(
            Arg::with_name("markdown-file")
                .value_name("FILE")
                .takes_value(true)
                .required(true)
                .help("Markdown file to be parsed. Use - to read from stdin instead."),
        )
}

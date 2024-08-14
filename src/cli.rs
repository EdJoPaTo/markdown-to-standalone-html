use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
pub enum SubCommands {
    /// Render the markdown to html.
    #[command()]
    Render {
        /// Template file to be used instead of the builtin one.
        ///
        /// Use the subcommand template to print the builtin template to stdout.
        #[arg(
        long,
        short,
        env,
        value_hint = ValueHint::FilePath,
        value_name = "FILE",
    )]
        template_file: Option<PathBuf>,

        /// Don't try to inline assets.
        ///
        /// Normally assets are inlined with monolith.
        /// When monolith is not in PATH a warning is shown.
        /// This warning is also suppressed with this flag.
        #[arg(long, short = 'i', env)]
        no_inline: bool,

        /// Markdown file to be parsed.
        /// Use - to read from stdin instead.
        #[arg(
        value_hint = ValueHint::FilePath,
        value_name = "FILE",
    )]
        markdown_file: PathBuf,
    },

    /// Print the included template to stdout.
    #[command()]
    Template,

    /// Only parse the markdown to html without any further modifications.
    #[command()]
    Raw {
        /// Markdown file to be parsed.
        /// Use - to read from stdin instead.
        #[arg(
            value_hint = ValueHint::FilePath,
            value_name = "FILE",
        )]
        markdown_file: PathBuf,
    },
}

#[derive(Debug, Parser)]
#[command(about, version, subcommand_negates_reqs = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommands: SubCommands,
}

#[test]
fn verify() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}

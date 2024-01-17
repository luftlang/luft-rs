use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub(crate) enum Cli {
    #[clap(alias = "c")]
    Compile {
        #[arg(short, long, default_value = "main.lu")]
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
use clap::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
        ArgGroup::new("token_input_method")
            .required(false)
            .args(&["token-path", "token"])
))]
pub struct Args {
    #[clap(long = "token-path", short = 'p')]
    pub token_path: Option<PathBuf>,

    #[clap(long, short)]
    pub token: Option<String>,
}

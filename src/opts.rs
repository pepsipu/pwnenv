use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    // Ubuntu version to run
    #[arg(short, long, default_value = "20.04")]
    pub ubuntu: String,
}

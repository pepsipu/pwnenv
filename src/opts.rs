use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    // Ubuntu version to run
    #[arg(short, long, default_value = "20.04")]
    pub ubuntu: String,
    // Container username
    #[arg(long, default_value = "pwn")]
    pub username: String,
    // Host port for container to listen on
    #[arg(short, long, default_value = "22014")]
    pub port: u16,
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

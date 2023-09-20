use clap::Parser;
use pwnenv::builder;
use pwnenv::docker;
use pwnenv::opts::Opts;

use futures::executor::block_on;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let dockerfile = builder::make_dockerfile(&opts.ubuntu);
    block_on(docker::build_docker(dockerfile))?;
    block_on(docker::launch_env())?;
    Ok(())
}

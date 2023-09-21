use clap::Parser;
use pwnenv::builder;
use pwnenv::docker;
use pwnenv::opts::Opts;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let dockerfile = builder::make_dockerfile(&opts.ubuntu);
    docker::build_docker(dockerfile, "silly_goof:latest").await?;
    docker::launch_env("meowcat", "silly_goof:latest").await?;
    Ok(())
}


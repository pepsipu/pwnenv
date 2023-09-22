use pwnenv::builder;
use pwnenv::docker;
use pwnenv::opts::Opts;
use pwnenv::ssh;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artifact = builder::build_artifact(
        &format!("ubuntu:{}", &pwnenv::OPTS.ubuntu),
        "silly_goof:latest",
    );
    docker::build_docker(artifact).await?;
    docker::launch_env("meowcat", "silly_goof:latest").await?;
    ssh::exec_ssh(&pwnenv::OPTS.username, pwnenv::OPTS.port);
    Ok(())
}

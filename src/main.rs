use pwnenv::builder;
use pwnenv::docker;
use pwnenv::modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artifact = builder::build_artifact(
        &format!("ubuntu:{}", &pwnenv::OPTS.ubuntu),
        "pwnenv:latest",
    );
    docker::build_docker(artifact).await?;
    docker::launch_env("pwnenv", "pwnenv:latest").await?;
    modules::ssh::exec_ssh(&pwnenv::OPTS.username, pwnenv::OPTS.port);
    Ok(())
}

#[macro_use]
extern crate lazy_static;
use clap::Parser;

pub mod builder;
pub mod config;
pub mod docker;
pub mod opts;
pub mod ssh;

pub mod modules;

lazy_static! {
    pub static ref DOCKER: bollard::Docker = get_docker();
    pub static ref OPTS: opts::Opts = opts::Opts::parse();
}

fn get_docker() -> bollard::Docker {
    let docker = bollard::Docker::connect_with_local_defaults();
    match docker {
        Ok(docker) => docker,
        Err(e) => {
            panic!("Failed to connect to docker: {}", e);
        }
    }
}

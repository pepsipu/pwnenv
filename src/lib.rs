#[macro_use]
extern crate lazy_static;

pub mod builder;
pub mod config;
pub mod docker;
pub mod opts;
pub mod ssh;
pub mod tools;

pub mod modules;

lazy_static! {
    pub static ref DOCKER: bollard::Docker = get_docker();
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

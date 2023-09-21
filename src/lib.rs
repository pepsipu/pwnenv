pub mod builder;
pub mod docker;
pub mod opts;
pub mod tools;
pub mod config;

#[macro_use]
extern crate lazy_static;

fn get_docker() -> bollard::Docker {
    let docker = bollard::Docker::connect_with_local_defaults();
    match docker {
        Ok(docker) => docker,
        Err(e) => {
            panic!("Failed to connect to docker: {}", e);
        }
    }
}

lazy_static! {
    pub static ref DOCKER: bollard::Docker = get_docker();
}

// lazy_static! {
//     pub static ref DOCKER: Mutex<Option<Docker>> = Mutex::new(None);
// }

// pub fn connect_docker() -> Result<Docker, Box<dyn std::error::Error>> {
//     let docker = Docker::connect_with_local_defaults()?;
//     Ok(docker)
// }

// pub fn get_docker() -> &'static Option<Docker> {
//     match &*DOCKER {
//         Some(docker) => &DOCKER,
//         None => {
//             let docker = connect_docker().unwrap();
//             *DOCKER = Some(docker);
//             &DOCKER
//         }
//     }
// }

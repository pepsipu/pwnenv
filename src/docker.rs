use bollard::container::{Config, CreateContainerOptions};
use bollard::image::BuildImageOptions;

use crate::builder;

use futures::StreamExt;
use snafu::{prelude::*, Whatever};

pub async fn build_docker(artifact: builder::Artifact) -> Result<(), Whatever> {
    let mut build_image = crate::DOCKER.build_image(
        BuildImageOptions {
            dockerfile: "Dockerfile",
            t: &artifact.image,
            ..Default::default()
        },
        None,
        Some(artifact.tarball.into()),
    );
    while let Some(build_result) = build_image.next().await {
        match build_result {
            Ok(build_result) => println!("{:?}", build_result),
            Err(e) => whatever!("{:?}", e),
        }
    }
    Ok(())
}

pub async fn launch_env(container: &str, image: &str) -> Result<(), Whatever> {
    // ensure container doesn't exist
    let _ = crate::DOCKER.stop_container(container, None).await;
    let _ = crate::DOCKER.remove_container(container, None).await;

    let options = Some(CreateContainerOptions {
        name: container,
        platform: None,
    });

    let config = Config {
        image: Some(image),
        exposed_ports: Some({
            let mut m = std::collections::HashMap::new();
            m.insert("22/tcp", std::collections::HashMap::new());
            m
        }),
        host_config: Some(bollard::models::HostConfig {
            port_bindings: Some(
                vec![(
                    "22/tcp".to_string(),
                    Some(vec![bollard::models::PortBinding {
                        host_ip: None,
                        host_port: Some("20221".to_string()),
                    }]),
                )]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        }),
        ..Default::default()
    };
    let response = crate::DOCKER.create_container(options, config).await;
    let id = match response {
        Ok(response) => response.id,
        Err(e) => {
            whatever!("Failed to create container: {}", e);
        }
    };
    println!("Created container: {}", id);
    let start_response = crate::DOCKER.start_container::<String>(&id, None).await;
    match start_response {
        Ok(_) => println!("Started container: {}", id),
        Err(e) => {
            whatever!("Failed to start container: {}", e);
        }
    }
    Ok(())
}

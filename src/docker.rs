use std::io::Write;

use bollard::container::{Config, CreateContainerOptions};
use bollard::image::{BuildImageOptions, ListImagesOptions};

use dockerfile::Dockerfile;

use futures::StreamExt;
use mktemp;
use snafu::{prelude::*, ResultExt, Whatever};
use tar;

pub async fn list_images() -> Result<(), Whatever> {
    let images = crate::DOCKER
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await;
    match images {
        Ok(images) => {
            println!("{:?}", images);
            Ok(())
        }
        Err(e) => {
            whatever!("Failed to list images: {}", e)
        }
    }
}

pub async fn build_docker(dockerfile: Dockerfile) -> Result<(), Whatever> {
    // please ignore blatant toctou
    let path = mktemp::Temp::new_file().unwrap();
    let mut tmp = std::fs::File::create(&path).unwrap();
    tmp.write_all(dockerfile.to_string().as_bytes()).unwrap();
    let mut builder = tar::Builder::new(Vec::new());
    println!("{:?}", dockerfile.to_string());
    builder
        .append_file("Dockerfile", &mut std::fs::File::open(&path).unwrap())
        .unwrap();
    builder.finish();
    let tar = builder.into_inner().unwrap();
    // println!("{:?}", tar);
    let mut build_image = crate::DOCKER.build_image(
        BuildImageOptions {
            dockerfile: "Dockerfile",
            t: "goofball",
            ..Default::default()
        },
        None,
        Some(tar.into()),
    );
    while let Some(build_result) = build_image.next().await {
        match build_result {
            Ok(build_result) => {
                println!("{:?}", build_result);
            }
            Err(e) => {
                whatever!("{:?}", e);
            }
        }
    }
    Ok(())
    // return build_image;
}

pub async fn launch_env() -> Result<(), Whatever> {
    let build_stream = build_docker(make_dockerfile()).await;

    let _ = crate::DOCKER
        .remove_container("my_new_container", None)
        .await;

    let options = Some(CreateContainerOptions {
        name: "my_new_container",
        platform: None,
    });

    let config = Config {
        image: Some("goofball:latest"),
        // cmd: Some(vec!["/bin/bash"]),
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

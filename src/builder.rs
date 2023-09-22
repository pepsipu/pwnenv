use dockerfile::{Dockerfile, DockerfileBuilder};

// list of functions that take a builder and return a builder

pub struct Builder {
    pub image: String,
    pub dockerfile: DockerfileBuilder,
    pub tar: tar::Builder<Vec<u8>>,
}

pub struct Artifact {
    pub dockerfile: Dockerfile,
    pub image: String,
    pub tarball: Vec<u8>,
}

pub fn add_file<W>(tar: &mut tar::Builder<W>, path: &str, content: &[u8])
where
    W: std::io::Write,
{
    let mut header = tar::Header::new_gnu();
    header.set_path(path).unwrap();
    header.set_size(content.len() as u64);
    header.set_cksum();
    tar.append(&header, content).unwrap();
}

impl Builder {
    pub fn new(base: String, image: String) -> Self {
        Builder {
            image,
            dockerfile: Dockerfile::base(base),
            tar: tar::Builder::new(Vec::new()),
        }
    }

    pub fn update_df<F>(mut self, f: F) -> Self
    where
        F: FnOnce(DockerfileBuilder) -> DockerfileBuilder,
    {
        self.dockerfile = f(self.dockerfile);
        self
    }

    pub fn build(mut self) -> Artifact {
        let dockerfile = self.dockerfile.finish();
        add_file(
            &mut self.tar,
            "Dockerfile",
            dockerfile.to_string().as_bytes(),
        );
        self.tar.finish().unwrap();
        Artifact {
            dockerfile,
            tarball: self.tar.into_inner().unwrap(),
            image: self.image,
        }
    }
}

pub fn build_artifact(base: &str, image: &str) -> Artifact {
    let builder = Builder::new(base.into(), image.into());
    crate::modules::MODULES
        .iter()
        .fold(builder, |builder, f| f(builder))
        .build()
}

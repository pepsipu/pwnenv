use std::path::Path;

use dockerfile::{Cmd, Copy, Dockerfile, DockerfileBuilder, Run};

use crate::ssh;

pub struct Builder {
    image: String,
    dockerfile: DockerfileBuilder,
    tar: tar::Builder<Vec<u8>>,
}

pub struct Artifact {
    pub dockerfile: Dockerfile,
    pub image: String,
    pub tarball: Vec<u8>,
}

fn add_file<W>(tar: &mut tar::Builder<W>, path: &str, content: &[u8])
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

    pub fn apt_update(self) -> Self {
        self.update_df(|df| df.push(Run::new("apt-get update")))
    }

    pub fn emit_ssh_install(self) -> Self {
        self.update_df(|df| {
            df.push(Run::new("apt-get install openssh-server -y"))
                .push(Run::new("mkdir /var/run/sshd"))
        })
    }

    pub fn emit_ssh_cmd(self) -> Self {
        self.update_df(|df| df.push(Cmd::new("[\"/usr/sbin/sshd\", \"-D\"]")))
    }

    pub fn setup_ssh_user(mut self, username: &str, ssh_key: &Path) -> Self {
        let ssh_key_str = ssh_key.file_name().unwrap().to_str().unwrap();

        self.tar
            .append_path_with_name(ssh_key, ssh_key_str)
            .unwrap();
        self.update_df(|df| {
            df.push(Run::new(format!("useradd -m {} -s /bin/bash", username)))
                .push(Run::new(format!("mkdir /home/{}/.ssh", username)))
                .push(Run::new(format!(
                    "chown -R {} /home/{}/.ssh",
                    username, username
                )))
                .push(Run::new(format!("chmod 700 /home/{}/.ssh", username)))
                .push(Copy::new(format!(
                    "{} /home/{}/.ssh/authorized_keys",
                    ssh_key_str, username
                )))
                .push(Run::new(format!(
                    "chown {} /home/{}/.ssh/authorized_keys && chmod 600 /home/{}/.ssh/authorized_keys",
                    username,
                    username, username
                )))
        })
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

pub fn build_artifact(ubuntu: &str, image: &str) -> Artifact {
    let builder = Builder::new(format!("ubuntu:{}", ubuntu), image.to_string());
    builder
        .apt_update()
        .emit_ssh_cmd()
        .emit_ssh_install()
        .setup_ssh_user("pwn", &ssh::get_ssh_pubkey())
        .build()
}

use dockerfile::{Cmd, Dockerfile, DockerfileBuilder, Run};

struct Builder {
    dockerfile: Option<DockerfileBuilder>,
}

impl Builder {
    pub fn new(base: String) -> Self {
        Self {
            dockerfile: Some(Dockerfile::base(base)),
        }
    }

    pub fn take_df(&mut self) -> DockerfileBuilder {
        return self.dockerfile.take().unwrap();
    }

    pub fn add_ssh_server(&mut self) {
        self.dockerfile = Some(
            self.take_df()
                .push(Run::new("apt-get install openssh-server -y"))
                .push(Run::new("mkdir /var/run/sshd")),
        );
    }

    pub fn build(&mut self) -> Dockerfile {
        return self.take_df().finish();
    }
}

pub fn make_dockerfile(ubuntu: &str) -> Dockerfile {
    let dockerfile = Dockerfile::base(format!("ubuntu:{}", ubuntu))
        // .push(Env::new("DEBIAN_FRONTEND noninteractive"))
        .push(Run::new("apt-get update"))
        .push(Run::new("apt-get install openssh-server -y"))
        .push(Run::new("mkdir /var/run/sshd"))
        .push(Cmd::new("[\"/usr/sbin/sshd\", \"-D\"]"))
        .finish();
    return dockerfile;
}

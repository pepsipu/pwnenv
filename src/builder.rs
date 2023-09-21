use dockerfile::{Cmd, Dockerfile, DockerfileBuilder, Run};
use ssh_key::private::{Ed25519Keypair, Ed25519PrivateKey};
use rand::rngs::StdRng;

struct Builder {
    dockerfile: DockerfileBuilder,
}

impl Builder {
    pub fn new(base: String) -> Self {
        Builder { dockerfile: Dockerfile::base(base) }
    }

    pub fn emit_ssh_install(self) {
            self.dockerfile
                .push(Run::new("apt-get install openssh-server -y"))
                .push(Run::new("mkdir /var/run/sshd"));
    }

    pub fn emit_ssh_cmd(self) {
        self.dockerfile.push(Cmd::new("[\"/usr/sbin/sshd\", \"-D\"]"));
    }

    pub fn generate_ssh_key(self) {
        let key = Ed25519Keypair::random(&mut rand::thread_rng());
        key.public.to_string();
    }

    pub fn build(self) -> Dockerfile {
        return self.dockerfile.finish();
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

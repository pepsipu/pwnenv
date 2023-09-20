use dockerfile::{Cmd, Dockerfile, Run};

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

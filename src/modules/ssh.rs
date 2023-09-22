use std::path::Path;

use crate::builder::Builder;
use crate::ssh;
use dockerfile::{Cmd, Copy, Run};

fn emit_ssh_install(builder: Builder) -> Builder {
    builder.update_df(|df| {
        df.push(Run::new("apt-get install openssh-server -y"))
            .push(Run::new("mkdir /var/run/sshd"))
    })
}

fn emit_ssh_cmd(builder: Builder) -> Builder {
    builder.update_df(|df| df.push(Cmd::new("[\"/usr/sbin/sshd\", \"-D\"]")))
}

fn setup_ssh_user(mut builder: Builder, username: &str, ssh_key: &Path) -> Builder {
    let ssh_key_str = ssh_key.file_name().unwrap().to_str().unwrap();

    builder
        .tar
        .append_path_with_name(ssh_key, ssh_key_str)
        .unwrap();
    builder.update_df(|df| {
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
                username, username, username
            )))
    })
}

pub fn add_ssh(builder: Builder) -> Builder {
    let ssh_key = ssh::get_ssh_pubkey();
    let builder = emit_ssh_install(builder);
    let builder = setup_ssh_user(builder, "pwn", &ssh_key);
    emit_ssh_cmd(builder)
}

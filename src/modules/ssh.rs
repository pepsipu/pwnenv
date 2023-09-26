use std::path::Path;

use crate::builder::Builder;
use dockerfile::{Cmd, Copy, Run};

use crate::config;
use std::fs::OpenOptions;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;

use std::io::Write;
use std::os::unix::prelude::OpenOptionsExt;

pub fn save_ssh_key() {
    let key =
        ssh_key::PrivateKey::random(&mut rand::thread_rng(), ssh_key::Algorithm::Ed25519).unwrap();
    let private_pem = key.to_openssh(ssh_key::LineEnding::LF).unwrap();
    let public_key = key.public_key().to_openssh().unwrap();

    // save to cfg
    let (private_key_path, public_key_path) = config::get_ssh_key_path();
    // Write with permissions
    let mut private_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600) // Set permissions to -rw-------
        .open(&private_key_path)
        .unwrap();

    private_file.write_all(private_pem.as_bytes()).unwrap();

    // Assuming public key doesn't need to be as restricted, but modify as necessary
    let mut public_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644) // Set permissions to -rw-r--r--
        .open(&public_key_path)
        .unwrap();

    public_file.write_all(public_key.as_bytes()).unwrap();
}

pub fn get_ssh_pubkey() -> PathBuf {
    let (_private_key_path, public_key_path) = config::get_ssh_key_path();
    if !public_key_path.exists() {
        save_ssh_key();
    }
    return public_key_path;
}

pub fn get_ssh_privkey() -> PathBuf {
    let (private_key_path, _public_key_path) = config::get_ssh_key_path();
    if !private_key_path.exists() {
        panic!(
            "Private key not found at {}.",
            private_key_path.to_str().unwrap()
        );
    }
    return private_key_path;
}

pub fn get_ssh_key_path() -> (PathBuf, PathBuf) {
    let cfg_dir = config::get_cfg_dir();
    let private_key_path = cfg_dir.join("id_ed25519");
    let public_key_path = cfg_dir.join("id_ed25519.pub");
    return (private_key_path, public_key_path);
}

// execute ssh pwn@localhost -p 20221 -i ~/.config/pwnenv/id_ed25519
pub fn exec_ssh(username: &str, port: u16) {
    let path = get_ssh_privkey();
    let path = path.to_str().unwrap();
    // also add -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null
    std::process::Command::new("ssh")
        .args(&[
            &format!("{}@localhost", username),
            "-p",
            &format!("{}", port),
            "-i",
            path,
            "-o",
            "StrictHostKeyChecking=no",
            "-o",
            "UserKnownHostsFile=/dev/null",
        ])
        .exec();
}


fn add_install(builder: Builder) -> Builder {
    builder.update_df(|df| {
        df.push(Run::new("apt-get install openssh-server -y"))
            .push(Run::new("mkdir /var/run/sshd"))
    })
}

fn add_cmd(builder: Builder) -> Builder {
    builder.update_df(|df| df.push(Cmd::new("[\"/usr/sbin/sshd\", \"-D\"]")))
}

fn add_key(mut builder: Builder, username: &str, ssh_key: &Path) -> Builder {
    let ssh_key_str = ssh_key.file_name().unwrap().to_str().unwrap();

    builder
        .tar
        .append_path_with_name(ssh_key, ssh_key_str)
        .unwrap();
    builder.update_df(|df| {
        df.push(Run::new(format!("mkdir /home/{}/.ssh", username)))
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
    let ssh_key = get_ssh_pubkey();
    let builder = add_install(builder);
    let builder = add_key(builder, &crate::OPTS.username, &ssh_key);
    add_cmd(builder)
}

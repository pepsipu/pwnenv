use crate::config;
use std::fmt::format;
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

// execute ssh pwn@localhost -p 20221 -i ~/.config/pwnenv/id_ed25519
pub fn exec_ssh(username: &str, port: u16) {
    let path = get_ssh_privkey();
    let path = path.to_str().unwrap();
    std::process::Command::new("ssh")
        .arg(format!("{}@localhost", username))
        .arg("-p")
        .arg(format!("{}", port))
        .arg("-i")
        .arg(path)
        .exec();
}

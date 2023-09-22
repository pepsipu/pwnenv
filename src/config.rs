use directories::ProjectDirs;
use std::path::PathBuf;

pub fn get_cfg_dir() -> PathBuf {
    let project_dirs = ProjectDirs::from("pw", "pepsipu", "pwnenv").unwrap();
    let cfg = project_dirs.config_dir();
    std::fs::create_dir_all(cfg).unwrap();
    return cfg.to_path_buf();
}

pub fn get_ssh_key_path() -> (PathBuf, PathBuf) {
    let cfg_dir = get_cfg_dir();
    let private_key_path = cfg_dir.join("id_ed25519");
    let public_key_path = cfg_dir.join("id_ed25519.pub");
    return (private_key_path, public_key_path);
}

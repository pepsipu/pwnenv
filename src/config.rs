use directories::ProjectDirs;
use std::path::PathBuf;

pub fn get_cfg_dir() -> PathBuf {
    let project_dirs = ProjectDirs::from("pw", "pepsipu", "pwnenv").unwrap();
    let cfg = project_dirs.config_dir();
    std::fs::create_dir_all(cfg).unwrap();
    return cfg.to_path_buf();
}



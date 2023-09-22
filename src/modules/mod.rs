use crate::builder::Builder;

mod apt;
mod ssh;

pub const MODULES: &[fn(Builder) -> Builder] = &[apt::add_apt, ssh::add_ssh];

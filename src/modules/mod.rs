use crate::builder::Builder;

mod base;
mod ssh;

pub const MODULES: &[fn(Builder) -> Builder] = &[base::add_base, ssh::add_ssh];

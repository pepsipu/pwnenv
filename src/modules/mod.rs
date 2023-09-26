use crate::builder::Builder;

mod base;
pub mod ssh;

pub const MODULES: &[fn(Builder) -> Builder] = &[
    base::add_base,
    // ssh::add_ssh,
    // the whole shebang
    |builder| {
        builder.update_df(|df| {
            df.push(dockerfile::Run::new("apt -y install curl"))
                .push(dockerfile::Run::new("curl -sSL https://sh.pepsi.pw | bash"))
        })
    },
];

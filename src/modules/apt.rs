use crate::builder::Builder;
use dockerfile::Run;

pub fn add_apt(builder: Builder) -> Builder {
    builder.update_df(|df| df.push(Run::new("apt-get update")))
}

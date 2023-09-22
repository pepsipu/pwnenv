use crate::builder::Builder;
use dockerfile::Run;

pub fn add_base(builder: Builder) -> Builder {
    builder.update_df(|df| {
        df.push(Run::new(format!(
            "useradd -m {} -s /bin/bash",
            &crate::OPTS.username
        )))
        .push(Run::new("apt-get update"))
    })
}

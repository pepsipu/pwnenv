use crate::builder::Builder;
use dockerfile::{Run, Env, Workdir};



pub fn add_base(builder: Builder) -> Builder {
    builder.update_df(|df| {
        df.push(Run::new(format!(
            "useradd -m {} -s /bin/bash",
            &crate::OPTS.username
        )))
        
        .push(Env::new("TZ America/Los_Angeles"))
        .push(Run::new("ln -fs /usr/share/zoneinfo/${TZ} /etc/localtime"))

        .push(Run::new("apt -y update"))
        .push(Run::new("apt -y install curl tzdata"))

        /*
        ENV LC_CTYPE C.UTF-8
ENV TERM xterm-256color

WORKDIR /pwn/ */
        .push(Env::new("LC_CTYPE C.UTF-8"))
        .push(Env::new("TERM xterm-256color"))
        .push(Workdir::new("/pwn/"))
        
    })
}

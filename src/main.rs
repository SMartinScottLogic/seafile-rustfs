use std::ffi::OsStr;

use clap::Parser;

use seafile_rustfs::SeafileFS;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Havvoric <havvoric@gmail.com>")]
struct Opts {
    server: String,
    username: String,
    password: String,
    mountpoint: String,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    log4rs::init_file("log4rs.yml", Default::default())?;

    let filesystem = SeafileFS::new(&opts.server, &opts.username, &opts.password);
    let options = ["-o", "rw", "-o", "fsname=seafile", "-a", "auto_mount"];
    let options = options.iter().map(|o| o.as_ref()).collect::<Vec<&OsStr>>();
    fuse_mt::mount(
        fuse_mt::FuseMT::new(filesystem, 1),
        &opts.mountpoint,
        &options,
    )?;
    Ok(())
}

use std::{path::PathBuf, process};

use clap:: Parser;

use crate::tapisfs::TapisFileSystem;

use fuse_mt::{mount, FuseMT};

pub mod tapisfs;

pub mod tapiscl;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// The Tapis Base URL to use
    #[arg(short, long)]
    base_url: String, 

    /// The Tapis system id to use
    #[arg(short, long)]
    system_id: String, 

    /// A valid Tapis JWT to use
    #[arg(short, long)]
    jwt: String, 

    /// Path
    #[arg(short, long)]
    mount_point: PathBuf,

}

fn main() {
    let args = Args::parse();
    let jwt_slice = &args.jwt[..10];

    // convert mount point to absolute path and make sure it exists
    if !args.mount_point.exists() {
        println!("The mount point ({:?}) doesn't exist! Create the directory first.", args.mount_point);
        process::exit(1);
    }
    println!("\n\nConfiguration\n=============\nBase URL: {}", args.base_url);
    println!("System ID: {}", args.system_id);
    println!("JWT (elided): {}...", jwt_slice);
    println!("Mount Point: {:?}", args.mount_point);

    // blocks until the fs is unmounted
    let _ = mount(fuse_mt::FuseMT::new(TapisFileSystem{ base_url: args.base_url, system_id: args.system_id, jwt: args.jwt }, 1), &args.mount_point, &[]);
}

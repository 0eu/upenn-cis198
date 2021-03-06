use std::{fs, io};
use std::path::PathBuf;
use structopt::StructOpt;
use std::env::{JoinPathsError, join_paths};
use std::any::Any;


#[derive(Debug, StructOpt)]
#[structopt(name = "Finder", about = "A command line utility for searching files with regexes")]
struct CLI {
    /// List of directories to search in.
    #[structopt(short, long, required = true, parse(from_os_str))]
    dirs: Vec<PathBuf>,

    /// List of patterns to use.
    #[structopt(short, long, required = true)]
    patterns: Vec<String>,

    /// Write results to output file instead of stdout.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Match on file size (in bytes, kilobytes, megabytes, etc.).
    #[structopt(short, long)]
    size: Option<usize>,

    /// Match files of at most a certain depth
    #[structopt(long)]
    depth: Option<u16>,

    /// Match directories in addition to files
    #[structopt(long)]
    with_dirs: Option<bool>,

    /// Match files with a given file types
    #[structopt(long)]
    extensions: Vec<String>,

    /// Match files with certain permissions set
    #[structopt(long)]
    permission_set: Option<u8>,
}

struct File {
    name: String,
    dir: String,
    size: u64,
    permission_set: u8,
}

impl File {
    fn from_path(path: PathBuf) -> Result<Self> {
        unimplemented!()
    }
}


fn main() {
    let args = CLI::from_args();
    println!("{:?}", args.dirs)
}

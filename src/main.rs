use clap::{ArgAction, Parser};
use std::process;
mod counter;

#[derive(Parser)]
#[clap(
    author = "sinasun",
    version = "1.0",
    about = "A simple program to count characters, words and lines in folders and files"
)]

struct Arguments {
    #[clap(help = "Path for a file or a folder to count")]
    path: String,

    #[clap(short, long, help = "Add characters counter", action=ArgAction::SetTrue)]
    characters: bool,

    #[clap(short, long, help = "Add words counter", action=ArgAction::SetTrue)]
    words: bool,

    #[clap(short, long, help = "Add lines counter", action=ArgAction::SetTrue)]
    lines: bool,

    #[clap(
        short,
        long,
        help = "Depth of subfolders to print, if not provided print all"
    )]
    print_depth: Option<u16>,

    #[clap(
        short,
        long,
        help = "Max depth of sub folders to perform counting. If not provided, stops when no more sub folder is remaining"
    )]
    max_depth: Option<u16>,
}

fn main() {
    let args = Arguments::parse();

    let mut counter_object = counter::Counter::build(&args.path).unwrap_or_else(|err| {
        println!("Problem reading the file or directory: {err}");
        process::exit(1);
    });
    counter_object.discover_directories();
}

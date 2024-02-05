use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "sinasun",
    version,
    about = "A simple program to count characters, words and lines in folder and files"
)]

struct Arguments {
    path: String,
}

fn main() {
    let args = Arguments::parse();

    println!("{}", args.path);
}

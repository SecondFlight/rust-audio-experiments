use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0")]
struct Opts {
   file: String,
}

fn main() {
    let opts = Opts::parse();

    println!("{}", opts.file);
}

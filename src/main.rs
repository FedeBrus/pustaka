use clap::Parser;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    input: String,

    #[arg(short, long, default_value_t = String::from("output.html"))]
    output: String,

    #[arg(short, long, default_value_t = String::from("style.css"))]
    style: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut file = File::open(args.input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{contents}");
    Ok(())
}

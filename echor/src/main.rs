use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    name: String,
    count: u8,
}

fn main() {
    println!("{:?}", std::env::args());
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}

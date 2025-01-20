use clap::{Arg, Command};
fn main() {
    let matches = Command::new("echor")
        .author("Memnoc, <memnochmod@gmail.com>")
        .version("1.0.0")
        .about("Rust echor")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();
    println!("{:#?}", matches);
}

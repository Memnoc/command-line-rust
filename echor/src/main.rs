use clap::Command;
fn main() {
    let _matches = Command::new("echor")
        .author("Memnoc, <memnochmod@gmail.com>")
        .version("1.0.0")
        .about("Rust echor")
        .help_template("{bin} {version}\nBy: {author}\n{about}\n\n{usage}\n\nOptions:\n{options}") // Custom template
        .get_matches();
}

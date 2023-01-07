use clap::{self, Parser};

#[derive(Parser)]
struct Opts {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Dump the UIA tree on the current desktop.
    Dump,
}

fn main() {
    let opts = Opts::parse();
    match opts.subcommand {
        Subcommand::Dump => println!("dumping…"),
    }
}

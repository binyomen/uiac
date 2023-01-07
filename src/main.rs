use {
    clap::{self, Parser},
    uiac::{dump, UiacResult},
    windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
};

#[derive(Parser)]
#[command(author, version, about)]
struct Opts {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Dump the UIA tree on the current desktop.
    Dump,
}

fn main() -> UiacResult<()> {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }?;

    let opts = Opts::parse();
    match opts.subcommand {
        Subcommand::Dump => dump(),
    }?;

    Ok(())
}

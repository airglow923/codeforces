mod div3;

use clap::Parser;
use tracing::Level;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.debug {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }

    div3::problem_2126c::solve();
}

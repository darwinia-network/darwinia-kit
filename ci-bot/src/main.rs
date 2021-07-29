mod cli;
use cli::Cli;

mod checker;

// --- crates.io ---
use structopt::StructOpt;

fn main() {
	let cli = Cli::from_args();

	dbg!(cli);
}

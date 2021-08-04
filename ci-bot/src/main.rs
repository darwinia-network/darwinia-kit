mod result;
use result::{Error as AnyError, Result as AnyResult};

mod cli;
use cli::Cli;

mod checker;
use checker::Check;

// --- std ---
use std::process;
// --- crates.io ---
use structopt::StructOpt;

fn main() {
	let cli = Cli::from_args();

	dbg!(&cli);

	match cli.checker.check() {
		Ok(exit_code) => process::exit(exit_code),
		Err(e) => {
			dbg!(e);

			process::exit(-1);
		}
	}
}

// --- crates.io ---
use structopt::StructOpt;
// --- ci-bot ---
use crate::checker::Checker;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub checker: Checker,
}

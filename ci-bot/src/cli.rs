// --- crates.io ---
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
	#[derive(Debug)]
	pub enum Chain {
		Darwinia,
		Crab,
		Pangolin
	}
}
impl Chain {
	fn rpc(&self) -> &str {
		match self {
			Chain::Darwinia => "https://rpc.darwinia.network",
			Chain::Crab => "https://crab-rpc.darwinia.network",
			Chain::Pangolin => "https//pangolin-rpc.darwinia.network",
		}
	}
}

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub checker: Checker,
}

#[derive(Debug, StructOpt)]
pub enum Checker {
	DefaultFeatures,
	StoragePrefix {
		#[structopt(long, name = "chain", case_insensitive = true, possible_values = &Chain::variants())]
		chain: Chain,
		#[structopt(default_value = "http://localhost:9933")]
		local_node: String,
	},
}

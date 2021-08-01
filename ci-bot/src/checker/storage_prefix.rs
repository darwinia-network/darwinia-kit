mod arg {
	// --- crates.io ---
	use structopt::clap::arg_enum;

	arg_enum! {
		#[derive(Debug)]
		pub enum Chain {
			Darwinia,
			Crab,
			Pangolin
		}
	}
	impl Chain {
		pub fn rpc_endpoint(&self) -> &str {
			match self {
				Chain::Darwinia => "https://rpc.darwinia.network",
				Chain::Crab => "https://crab-rpc.darwinia.network",
				Chain::Pangolin => "https://pangolin-rpc.darwinia.network",
			}
		}
	}
}
use arg::*;

// --- std ---
use std::{
	convert::TryFrom,
	path::PathBuf,
	process::{Command, Stdio},
	thread,
	time::Duration,
};
// --- crates.io ---
use colored::Colorize;
use serde_json::Value;
use structopt::StructOpt;
use submetadatan::{
	parity_scale_codec::Decode, Metadata, RuntimeMetadataPrefixed, Storage, Storages,
};
use subrpcer::{isahc::ReadResponseExt, state};
// --- ci-bot ---
use crate::{checker::Check, AnyError, AnyResult};

#[derive(Debug, StructOpt)]
pub struct Checker {
	#[structopt(long, case_insensitive = true, possible_values = &Chain::variants())]
	chain: Chain,
	#[structopt(long)]
	exec: PathBuf,
}
impl Check for Checker {
	fn check(&self) -> AnyResult<()> {
		const LOCAL_NODE_RPC_END_POINT: &str = "http://localhost:9933";

		fn differentiate<'a, T, V, F>(a: &'a [T], b: &'a [T], f: F) -> Vec<&'a V>
		where
			V: 'a + PartialEq,
			F: Fn(&'a T) -> &'a V + Copy,
		{
			b.iter()
				.map(f)
				.filter_map(|x| {
					if a.iter().map(f).find(|y| x == *y).is_some() {
						None
					} else {
						Some(x)
					}
				})
				.collect()
		}

		println!("Spawning Local Node...");
		let mut local_node = Command::new(&self.exec)
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.args(&["--chain", &format!("{}-dev", self.chain), "--tmp"])
			.spawn()?;

		thread::sleep(Duration::from_secs(3));

		let local_storages = fetch_storages(LOCAL_NODE_RPC_END_POINT)?;
		let chain_storages = fetch_storages(self.chain.rpc_endpoint())?;

		println!("Pallet Level Storage Changes:");
		for storage_prefix in
			differentiate(&chain_storages, &local_storages, |storages: &Storages| {
				&storages.prefix
			}) {
			println!("{}", format!("+ {}", storage_prefix).green());
		}
		for storage_prefix in
			differentiate(&local_storages, &chain_storages, |storages: &Storages| {
				&storages.prefix
			}) {
			println!("{}", format!("- {}", storage_prefix).red());
		}

		local_node.kill()?;

		Ok(())
	}
}

fn fetch_storages(uri: impl AsRef<str>) -> AnyResult<Vec<Storages>> {
	let metadata = {
		let mut response = subrpcer::send_rpc(uri, state::get_metadata())?.json::<Value>()?;
		let hex_codec_metadata = response
			.get_mut("result")
			.map(|v| v.take())
			.ok_or(AnyError::Custom(""))?
			.as_str()
			.map(ToOwned::to_owned)
			.ok_or(AnyError::Custom(""))?;
		let codec_metadata =
			array_bytes::hex2bytes(hex_codec_metadata).map_err(|_| AnyError::Custom(""))?;
		let metadata_prefixed = RuntimeMetadataPrefixed::decode(&mut &*codec_metadata)?;
		let metadata = Metadata::try_from(metadata_prefixed.1)?;

		metadata
	};
	let storages = metadata
		.modules
		.into_iter()
		.filter_map(|module| module.storages)
		.collect();

	Ok(storages)
}

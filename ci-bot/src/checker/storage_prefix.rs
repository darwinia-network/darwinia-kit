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
	io::{BufRead, BufReader},
	path::PathBuf,
	process::{Child, Command, Stdio},
};
// --- crates.io ---
use colored::Colorize;
use serde_json::Value;
use structopt::StructOpt;
use submetadatan::{
	parity_scale_codec::Decode, Metadata, RuntimeMetadataPrefixed, Storage, Storages,
};
use subrpcer::{client::u as ureq, state};
// --- ci-bot ---
use crate::{checker::Check, AnyError, AnyResult};

#[derive(Debug, StructOpt)]
pub struct Checker {
	#[structopt(long, case_insensitive = true, possible_values = &Chain::variants())]
	chain: Chain,
	#[structopt(long)]
	exec: PathBuf,
}
impl Checker {
	fn spawn_local_node(&self) -> AnyResult<Child> {
		println!("Spawning Local Node...");

		let mut local_node = Command::new(&self.exec)
			.stdout(Stdio::null())
			.stderr(Stdio::piped())
			.args(&["--chain", &format!("{}-dev", self.chain), "--tmp"])
			.spawn()?;
		let output = BufReader::new(local_node.stderr.take().ok_or(AnyError::Custom(""))?);

		for line in output.lines().filter_map(Result::ok) {
			if line.contains("Idle") {
				break;
			}
		}

		Ok(local_node)
	}

	fn fetch_storages(&self) -> AnyResult<(Vec<Storages>, Vec<Storages>)> {
		const LOCAL_NODE_RPC_END_POINT: &str = "http://localhost:9933";

		fn fetch_from(uri: impl AsRef<str>) -> AnyResult<Vec<Storages>> {
			let metadata = {
				let mut response =
					ureq::send_rpc(uri, state::get_metadata())?.into_json::<Value>()?;
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

		let local_storages = fetch_from(LOCAL_NODE_RPC_END_POINT)?;
		let chain_storages = fetch_from(self.chain.rpc_endpoint())?;

		Ok((local_storages, chain_storages))
	}

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

	fn check_pallet(local_storages: &[Storages], chain_storages: &[Storages]) {
		println!("Pallet Level Storage Change(s):");

		for storage_prefix in
			Self::differentiate(&local_storages, &chain_storages, |storages: &Storages| {
				&storages.prefix
			}) {
			println!("{}", format!("- Pallet: `{}`", storage_prefix).red());
		}
		for storage_prefix in
			Self::differentiate(&chain_storages, &local_storages, |storages: &Storages| {
				&storages.prefix
			}) {
			println!("{}", format!("+ Pallet: `{}`", storage_prefix).green());
		}
	}

	fn check_item(local_storages: &[Storages], chain_storages: &[Storages]) {
		println!("Item Level Storage Change(s):");

		for (local_storages, chain_storages) in local_storages.iter().filter_map(|local_storages| {
			if let Some(chain_storages) = chain_storages
				.iter()
				.find(|chain_storages| &local_storages.prefix == &chain_storages.prefix)
			{
				Some((local_storages, chain_storages))
			} else {
				None
			}
		}) {
			for storage in Self::differentiate(
				&local_storages.items,
				&chain_storages.items,
				|storage: &Storage| storage,
			) {
				println!(
					"{}",
					format!(
						"- Pallet: `{}`, Item: `{:?}`",
						chain_storages.prefix, storage
					)
					.red()
				);
			}
			for storage in Self::differentiate(
				&chain_storages.items,
				&local_storages.items,
				|storage: &Storage| storage,
			) {
				println!(
					"{}",
					format!(
						"+ Pallet: `{}`, Item: `{:?}`",
						local_storages.prefix, storage
					)
					.green()
				);
			}
		}
	}
}
impl Check for Checker {
	fn check(&self) -> AnyResult<i32> {
		let mut local_node = self.spawn_local_node()?;
		let (local_storages, chain_storages) = self.fetch_storages()?;

		Self::check_pallet(&local_storages, &chain_storages);
		Self::check_item(&local_storages, &chain_storages);

		local_node.kill()?;

		Ok(0)
	}
}

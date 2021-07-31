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
	path::PathBuf,
	process::{Command, Stdio},
};
// --- crates.io ---
use serde_json::Value;
use structopt::StructOpt;
use submetadatan::{parity_scale_codec::Decode, RuntimeMetadata, RuntimeMetadataPrefixed};
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
		let mut exec = Command::new(&self.exec)
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.args(&["--chain", &self.chain.to_string(), "--tmp"])
			.spawn()?;
		let metadata = {
			let mut response =
				subrpcer::send_rpc(self.chain.rpc_endpoint(), state::get_metadata())?
					.json::<Value>()?;
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
			let metadata = if let RuntimeMetadata::V12(metadata) = metadata_prefixed.1 {
				metadata
			} else {
				unimplemented!()
			};

			metadata
		};
		let storage_prefixes = metadata
			.modules
			.into_iter()
			.filter_map(|module| {
				module.storage.map(|storage| {
					storage
						.entries
						.iter()
						.map(|entry| {
							format!(
								"{}{}: {}",
								storage.prefix,
								entry.name,
								substorager::hex_storage_key_with_prefix(
									"0x",
									&storage.prefix,
									&entry.name
								)
							)
						})
						.collect::<Vec<_>>()
				})
			})
			.flatten()
			.collect::<Vec<_>>();

		dbg!(storage_prefixes);

		exec.kill()?;

		Ok(())
	}
}

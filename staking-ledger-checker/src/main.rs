// --- std ---
use std::fs::File;
// --- crates.io ---
use colored::Colorize;
use serde::{Deserialize, Deserializer};

fn main() {
	for ledger in
		serde_json::from_reader::<_, Vec<StakingLedger>>(File::open("data.json").unwrap()).unwrap()
	{
		let StakingLedger {
			ref stash,
			active_ring,
			active_deposit_ring,
			ref deposit_items,
			ref ring_staking_lock,
			active_kton,
			ref kton_staking_lock,
		} = ledger;
		let ss58_address = subcryptor::into_ss58_address(array_bytes::bytes_unchecked(stash), 18);
		let mut error = false;

		if active_ring != ring_staking_lock.staking_amount {
			error = true;

			eprintln!(
				"active ring: {}, locked ring: {}",
				active_ring.to_string().red(),
				ring_staking_lock.staking_amount.to_string().red()
			);
		}
		if active_ring < active_deposit_ring {
			error = true;

			eprintln!(
				"active ring: {}, active deposit ring: {}",
				active_ring.to_string().red(),
				active_deposit_ring.to_string().red(),
			);
		}
		if active_deposit_ring != deposit_items.iter().map(|d_i| d_i.value).sum() {
			error = true;

			eprintln!(
				"active deposit ring: {}, deposit items: {}",
				active_deposit_ring.to_string().red(),
				format!("{:#?}", deposit_items).red()
			);
		}
		if active_kton != kton_staking_lock.staking_amount {
			error = true;

			eprintln!(
				"active kton: {}, locked kton: {}",
				active_kton.to_string(),
				kton_staking_lock.staking_amount.to_string().red()
			);
		}
		if error {
			eprintln!(
				"stash public key: {}\nstash ss58 address: {}\n---",
				stash.to_string().yellow(),
				ss58_address.to_string().yellow()
			);
		}
	}
}

#[derive(Debug, Deserialize)]
struct StakingLedger {
	stash: String,

	#[serde(deserialize_with = "str_to_u128")]
	active_ring: u128,
	#[serde(deserialize_with = "str_to_u128")]
	active_deposit_ring: u128,
	#[serde(deserialize_with = "none_to_empty_vec")]
	deposit_items: Vec<DepositItem>,
	ring_staking_lock: StakingLock,

	#[serde(deserialize_with = "str_to_u128")]
	active_kton: u128,
	kton_staking_lock: StakingLock,
}

#[derive(Debug, Deserialize)]
struct DepositItem {
	start_time: u64,
	expire_time: u64,
	#[serde(deserialize_with = "str_to_u128")]
	value: u128,
}

#[derive(Debug, Deserialize)]
struct StakingLock {
	#[serde(deserialize_with = "str_to_u128")]
	staking_amount: u128,
	#[serde(deserialize_with = "none_to_empty_vec")]
	unbondings: Vec<Unbonding>,
}

#[derive(Debug, Deserialize)]
struct Unbonding {
	#[serde(deserialize_with = "str_to_u128")]
	amount: u128,
	moment: u64,
}

fn str_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(String::deserialize(deserializer)?.parse().unwrap())
}

fn none_to_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
	D: Deserializer<'de>,
	T: Deserialize<'de>,
{
	if let Some(v) = <Option<Vec<T>>>::deserialize(deserializer)? {
		Ok(v)
	} else {
		Ok(vec![])
	}
}

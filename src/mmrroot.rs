use std::env;

use ckb_merkle_mountain_range::{MMR, util::MemStore};
use darwinia_misc_toolset::MMRMerge;
use sp_core::H256;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn gen_mmr(hashes: Vec<H256>){
    let store = MemStore::default();
    let mut mmr = MMR::<_, MMRMerge, _>::new(0, &store);
    for hash in hashes {
        mmr.push(hash).unwrap();
    }
    println!("mmr_size: {:?}", mmr.mmr_size());
    println!("mmr_root: {:?}", mmr.get_root().unwrap());
    mmr.commit().expect("commit changes");
}

fn read_hashes<P>(filename: P) -> Vec<H256>
    where P: AsRef<Path>, {
    let mut result = vec![];

    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(l) = line {
            let h = str_to_h256(&l).unwrap();
            result.push(h);
        }
    }

    result
}

fn str_to_h256(line: &str) -> Result<H256> {
    if line.len() != 66 {
        Err(anyhow!(MmrRootError::WrongHashLength))
    } else {
        Ok(H256::from_slice(
            &hex::decode(&line[2..]).unwrap()[..]
        ))
    }
}

// https://ropsten.etherscan.io/uncle/0x0aae3601cda65335e8234866c482e7ea1d2dccf94332926e7d62a486f70ea2aa
// https://ropsten.etherscan.io/block/0x7848e122f3f665a1169c3a7880cbff7aa584879f97fbedc02b2a146477aee66f
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let file = &args[1];
//     let hashes = read_hashes(file);
//     gen_mmr(hashes);
// }

use std::io::prelude::*;

use anyhow::{Result, anyhow};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MmrRootError {
    #[error("the input block hash length is wrong")]
    WrongHashLength,
    #[error("unknown error")]
    Unknown,
}

pub fn main() -> Result<()> {
    let store = MemStore::default();
    let mut mmr = MMR::<_, MMRMerge, _>::new(0, &store);

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        println!("> index: {}, {}", i, &line);
        match str_to_h256(&line) {
            Ok(h) =>  {
                mmr.push(h)?;
                println!("size: {}, root: {:?}", mmr.mmr_size(), mmr.get_root()?);
            },
            Err(e) => println!("{}", e.to_string())
        }

    }
    Ok(())
}
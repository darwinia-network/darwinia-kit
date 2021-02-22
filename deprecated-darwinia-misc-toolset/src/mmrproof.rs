#[macro_use]
extern crate log;
use darwinia_misc_toolset::{convert, Result};
use sp_core::H256;
use std::env;

#[cfg(feature = "ssl")]
use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
#[cfg(feature = "ssl")]
use ws::util::TcpStream;

use ckb_merkle_mountain_range::leaf_index_to_pos;

// fn get_siblings(left_leaf_pos: u64, right_leaf_pos: u64, mmr_size: u64) {
//     let mut height = 1;
//     let mut new = right_leaf_pos + 1;
//     while new < (mmr_size - 1) {
//         (2 << height) - 1
//     }
// }

#[cfg(feature = "ssl")]
struct Client {
	out: ws::Sender,
	leaf: (u64, String),
}

#[cfg(feature = "ssl")]
impl ws::Handler for Client {
	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		// println!("msg = {}", msg);
		let text = msg.into_text()?;
		let parsed = json::parse(&text).unwrap();
		let id = parsed["id"].as_u32().unwrap();
		match id {
			0 => {
				let leaf = &parsed["result"].as_str().unwrap()[2..];
				self.leaf.1 = String::from(leaf);
				Ok(())
			}
			1 => {
				let mmr_size = parsed["result"]["mmrSize"].as_str().unwrap();
				let mmr_proof = parsed["result"]["proof"].as_str().unwrap();
				let proof = mmr_proof[1..mmr_proof.len() - 1]
					.split(", ")
					.collect::<Vec<&str>>()
					.iter()
					.map(|&x| String::from(&x[2..]))
					.collect::<Vec<String>>();

				let leaves = vec![(
					leaf_index_to_pos(self.leaf.0),
					H256::from_slice(&hex::decode(&self.leaf.1).unwrap()[..]),
				)];
				let proof: Vec<H256> = proof
					.iter()
					.map(|x| H256::from_slice(&hex::decode(x).unwrap()[..]))
					.collect();

				println!("--- original ---");
				println!("  mmr_size: {}", mmr_size);
				println!("  leaves  : {:?}", leaves.clone());
				println!("  proof   : {:?}\n", proof);

				// let (left_pos, right_pos) = if self.leaf.0.is_even() {
				//     let left_leaf_pos = leaf_index_to_pos(self.leaf.0);
				//     let right_leaf_pos = left_leaf_pos + 1;
				//     get_siblings(left_leaf_pos, right_leaf_pos, mmr_size);
				//     (left_index, right_index)
				// } else {
				//     let right_index = self.leaf.0;
				//     let left_index = right_index - 1;
				//     (left_index, right_index)
				// };
				//
				// let opponent = if leaf_index.is_even() { leaf_index + 1 } else { leaf_index - 1 };

				let converted_proof =
					convert(leaves.clone(), mmr_size.parse().unwrap(), proof).unwrap();
				println!("--- converted ---");
				println!("  mmr_size: {}", converted_proof.mmr_size);
				println!("  leaves  : {:?}", leaves);
				println!("  siblings: {:?}", converted_proof.siblings);
				println!("  peaks   : {:?}", converted_proof.peaks);

				self.out.close(ws::CloseCode::Normal)
			}
			_ => self.out.close(ws::CloseCode::Normal),
		}
	}

	fn upgrade_ssl_client(
		&mut self,
		sock: TcpStream,
		_: &url::Url,
	) -> ws::Result<SslStream<TcpStream>> {
		let mut builder = SslConnector::builder(SslMethod::tls()).map_err(|e| {
			ws::Error::new(
				ws::ErrorKind::Internal,
				format!("Failed to upgrade client to SSL: {}", e),
			)
		})?;
		builder.set_verify(SslVerifyMode::empty());

		let connector = builder.build();
		connector
			.configure()
			.unwrap()
			.use_server_name_indication(false)
			.verify_hostname(false)
			.connect("", sock)
			.map_err(From::from)
	}
}

#[cfg(feature = "ssl")]
fn main() -> Result<()> {
	// Setup logging
	env_logger::init();

	let args: Vec<String> = env::args().collect();
	let target_index = &args[1];
	let last_index = &args[2];
	let url = if args.len() != 4 {
		String::from("wss://cc1.darwinia.network")
	} else {
		String::from(format!("wss://{}.darwinia.network", args[3].clone()))
	};

	if let Err(error) = ws::connect(&url[..], |out| {
		let msg1 = format!("{{\"jsonrpc\": \"2.0\", \"method\": \"chain_getBlockHash\", \"params\": [{}], \"id\": 0}}", target_index);
		if let Err(_) = out.send(msg1) {
			error!("Websocket couldn't queue an initial message.")
		} else {
			info!("Message sent.")
		}

		let msg2 = format!("{{\"jsonrpc\": \"2.0\", \"method\": \"headerMMR_genProof\", \"params\": [{}, {}], \"id\": 1}}", target_index, last_index);
		if let Err(_) = out.send(msg2) {
			error!("Websocket couldn't queue an initial message.")
		} else {
			info!("Message sent.")
		}

		Client {
			out,
			leaf: (target_index.parse().unwrap(), String::from("")),
		}
	}) {
		error!("Failed to create WebSocket due to: {:?}", error);
	}

	Ok(())
}
#[cfg(not(feature = "ssl"))]
fn main() {
	println!("SSL feature is not enabled.")
}

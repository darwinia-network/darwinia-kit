
use core::marker::PhantomData;
use primitives::{
    chain::eth::{EthereumReceiptProofThing, HeaderStuff, RedeemFor},
    frame::{
        collective::{ExecuteCallExt, MembersStoreExt},
        ethereum::{
            backing::{RedeemCallExt, VerifiedProofStoreExt},
            game::{EthereumRelayerGame, PendingHeadersStoreExt, ProposalsStoreExt},
            relay::{
                ApprovePendingHeader, ConfirmedBlockNumbersStoreExt, RejectPendingHeader,
                SubmitProposalCallExt,
            },
        },
        sudo::{KeyStoreExt, SudoCallExt},
    },
    runtime::DarwiniaRuntime,
};
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::{
    sp_core::{Encode, Pair as PairTrait},
    Client, ClientBuilder, PairSigner,
};
use web3::types::H256;
pub type Result<T> = std::result::Result<T, std::error::Error>;

// Types
type PendingHeader = <DarwiniaRuntime as EthereumRelayerGame>::PendingHeader;
type RelayProposal = <DarwiniaRuntime as EthereumRelayerGame>::RelayProposal;

/// Account Role
#[derive(PartialEq, Eq)]
pub enum Role {
    /// Sudo Account
    Sudo,
    /// Council Member
    Council,
    /// Normal Account
    Normal,
}

/// Dawrinia API
pub struct Darwinia {
    client: Client<DarwiniaRuntime>,
    /// Keyring signer
    pub signer: PairSigner<DarwiniaRuntime, Pair>,
    /// Account Role
    pub role: Role,
}

impl Darwinia {
    /// New darwinia API
    pub async fn new(seed: String, node: String) -> Result<Darwinia> {
        let pair = Pair::from_string(&seed, None).unwrap();
        let signer = PairSigner::<DarwiniaRuntime, Pair>::new(pair);
        let client = ClientBuilder::<DarwiniaRuntime>::new()
            .set_url(&node)
            .build()
            .await?;

        let pk = signer.signer().public().to_string();
        let sudo = client.key(None).await?.to_string();
        let council = client.members(None).await?;

        Ok(Darwinia {
            client,
            signer,
            role: if sudo == pk {
                Role::Sudo
            } else if council.iter().any(|cpk| cpk.to_string() == pk) {
                Role::Council
            } else {
                Role::Normal
            },
        })
    }

}

#[tokio::main]
async fn main() {
    // let darwinia = Darwinia::new(&config).await.unwrap();
    //
    // let x = darwinia.last_confirmed().await.unwrap();
    // println!("{:?}", x);
}

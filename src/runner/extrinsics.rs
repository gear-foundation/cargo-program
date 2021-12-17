use anyhow::Result;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, PairSigner};

#[subxt::subxt(runtime_metadata_path = "node-metadata.scale")]
pub mod node {}

fn alice() -> PairSigner<node::DefaultConfig, sp_keyring::sr25519::sr25519::Pair> {
    PairSigner::new(AccountKeyring::Alice.pair())
}

async fn node_api() -> Result<node::RuntimeApi<node::DefaultConfig>> {
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<node::RuntimeApi<node::DefaultConfig>>();
    Ok(api)
}

pub async fn submit_program(
    code: Vec<u8>,
    salt: Vec<u8>,
    init_payload: Vec<u8>,
    gas_limit: u64,
    value: u128,
) -> Result<()> {
    let api = node_api().await?;
    let signer = alice();

    api.tx()
        .gear()
        .submit_program(code, salt, init_payload, gas_limit, value)
        .sign_and_submit(&signer)
        .await?;

    Ok(())
}

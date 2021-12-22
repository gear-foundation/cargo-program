use anyhow::Result;
use sp_keyring::{sr25519::sr25519::Pair, AccountKeyring};
use subxt::{ClientBuilder, PairSigner};
use tokio::runtime::Runtime;

#[subxt::subxt(runtime_metadata_path = "node-metadata.scale")]
mod gear_node {}
use gear_node::{DefaultConfig, RuntimeApi};

pub struct Node {
    url: Option<String>,
    signer: PairSigner<DefaultConfig, Pair>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            url: None,
            signer: Self::alice(),
        }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    fn alice() -> PairSigner<DefaultConfig, Pair> {
        PairSigner::new(AccountKeyring::Alice.pair())
    }

    async fn api(&self) -> Result<RuntimeApi<DefaultConfig>> {
        let api_builder = ClientBuilder::new();
        let api_builder = if let Some(url) = &self.url {
            api_builder.set_url(url)
        } else {
            api_builder
        };
        let api = api_builder
            .build()
            .await?
            .to_runtime_api::<RuntimeApi<DefaultConfig>>();
        Ok(api)
    }

    pub fn submit_program(
        &self,
        code: Vec<u8>,
        salt: Vec<u8>,
        init_payload: Vec<u8>,
        gas_limit: u64,
        value: u128,
    ) -> Result<()> {
        let rt = Runtime::new()?;
        rt.block_on(async {
            let api = self.api().await?;
            api.tx()
                .gear()
                .submit_program(code, salt, init_payload, gas_limit, value)
                .sign_and_submit(&self.signer)
                .await?;
            Ok(())
        })
    }
}

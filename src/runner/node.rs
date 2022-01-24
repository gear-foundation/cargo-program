use anyhow::Result;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, DefaultConfig, DefaultExtra, PairSigner};
use tokio::runtime::Runtime;

#[subxt::subxt(runtime_metadata_path = "node-metadata.scale")]
mod gear_node {}
use gear_node::RuntimeApi;

pub struct Node {
    url: Option<String>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            url: None,
        }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    async fn api(&self) -> Result<RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>> {
        let api_builder = ClientBuilder::new();
        let api_builder = if let Some(url) = &self.url {
            api_builder.set_url(url)
        } else {
            api_builder
        };
        let api = api_builder
            .build()
            .await?
            .to_runtime_api();//::<RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();
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
        let signer = PairSigner::new(AccountKeyring::Alice.pair());

        let rt = Runtime::new()?;
        rt.block_on(async {
            let api = self.api().await?;
            api.tx()
                .gear()
                .submit_program(code, salt, init_payload, gas_limit, value)
                .sign_and_submit(&signer)
                .await?;
            Ok(())
        })
    }
}

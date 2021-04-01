use log::info;
use std::path::Path;
use std::time::SystemTime;
use web3::api::Web3;
use web3::contract::Contract;
use web3::types::{Address, U256};

const NUM_CONFIRMATIONS: usize = 0;
const TOKEN_BASE_URI: &str = "https://nft-ptr.notnow.dev/";

pub struct NftPtrLib<T: web3::Transport> {
    web3: Web3<T>,
    pub account: Address,
    token_contract: Option<Contract<T>>,
}

impl<T: web3::Transport> NftPtrLib<T> {
    pub fn new(transport: T) -> NftPtrLib<T> {
        let web3 = web3::Web3::new(transport);
        NftPtrLib {
            web3: web3,
            account: Address::zero(),
            token_contract: None,
        }
    }
    pub async fn initialize(&mut self) {
        self.check_not_prod().await;
        self.account = self.web3.personal().list_accounts().await.unwrap()[0];
        info!("Account: {}", self.account);
        self.deploy_token_contract().await;
    }
    async fn check_not_prod(&self) {
        let version = self.web3.net().version().await.unwrap();
        info!("Connected to {} network", version);
        if version == "1" {
            panic!("Cowardly refusing to run on mainnet and waste real \"money\"");
        }
    }
    async fn deploy_token_contract(&mut self) {
        // rust-web3/examples/contract.rs
        // TODO(zhuowei): understand this
        let my_account = self.account;
        let bytecode = include_str!("../../../contracts/out/NftPtrToken.code");
        let contract = Contract::deploy(
            self.web3.eth(),
            include_bytes!("../../../contracts/out/NftPtrToken.json"),
        )
        .unwrap()
        .confirmations(NUM_CONFIRMATIONS)
        .options(web3::contract::Options::with(|opt| {
            // TODO(zhuowei): why does leaving this uncommented give me
            // "VM Exception while processing transaction: revert"
            //opt.value = Some(5.into());
            //opt.gas_price = Some(5.into());
            opt.gas = Some(6_000_000.into());
        }))
        .execute(
            bytecode,
            (
                // see NftPtrToken.sol's constructor
                /*name*/
                format!(
                    "NftPtrToken_{}_{}",
                    Path::new(&std::env::args().nth(0).unwrap())
                        .file_name()
                        .unwrap()
                        .to_string_lossy(),
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                ),
                /*symbol*/
                "NFT".to_owned(),
                /*baseTokenURI*/
                TOKEN_BASE_URI.to_owned(),
            ),
            my_account,
        )
        .await
        .unwrap();
        self.token_contract = Some(contract);
    }
    pub async fn mint_token(&self, address: Address, token_id: U256, token_uri: &str) {
        self.token_contract
            .as_ref()
            .unwrap()
            .call_with_confirmations(
                "mintId",
                (address, token_id, token_uri.to_owned()),
                self.account,
                web3::contract::Options::with(|opt| {
                    opt.gas = Some(1_000_000.into());
                }),
                NUM_CONFIRMATIONS,
            )
            .await
            .unwrap();
    }
}

pub async fn make_nft_ptr_lib_ipc() -> NftPtrLib<web3::transports::Ipc> {
    // TODO(zhuowei): don't hardcode this
    let transport = web3::transports::Ipc::new("TODOTODO").await.unwrap();
    NftPtrLib::new(transport)
}

pub fn make_nft_ptr_lib_localhost() -> NftPtrLib<web3::transports::Http> {
    let transport = web3::transports::Http::new("http://127.0.0.1:7545").unwrap();
    NftPtrLib::new(transport)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

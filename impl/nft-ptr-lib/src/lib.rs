use log::info;
use web3::api::Web3;
use web3::contract::Contract;
use web3::types::{Address, U256};

pub struct NftPtrLib<T: web3::Transport> {
    web3: Web3<T>,
    account: Address,
}

impl<T: web3::Transport> NftPtrLib<T> {
    pub fn new(transport: T) -> NftPtrLib<T> {
        let web3 = web3::Web3::new(transport);
        NftPtrLib {
            web3: web3,
            account: Address::zero(),
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
        .confirmations(0)
        .options(web3::contract::Options::with(|opt| {
            //opt.value = Some(5.into());
            opt.gas_price = Some(5.into());
            opt.gas = Some(3_000_000.into());
        }))
        .execute(
            bytecode,
            (
                // TODO(zhuowei): add updated arguments for constructor
                //U256::from(1_000_000_u64),
                //"My Token".to_owned(),
                //3u64,
                //"MT".to_owned(),
            ),
            my_account,
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

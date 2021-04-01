use log::info;
use web3::api::Web3;
pub struct NftPtrLib<T: web3::Transport> {
    web3: Web3<T>,
}

impl<T: web3::Transport> NftPtrLib<T> {
    pub async fn initialize(&mut self) {
        self.check_not_prod().await;
        self.deploy_token_contract().await;
    }
    async fn check_not_prod(&self) {
        let version = self.web3.net().version().await.unwrap();
        info!("Connected to {} network", version);
        if version == "main" {
            panic!("Cowardly refusing to run on mainnet and waste real \"money\"");
        }
    }
    async fn deploy_token_contract(&mut self) {}
}

pub async fn make_nft_ptr_lib_ipc() -> NftPtrLib<web3::transports::Ipc> {
    // TODO(zhuowei): don't hardcode this
    let transport = web3::transports::Ipc::new("TODOTODO").await.unwrap();
    let web3 = Web3::new(transport);
    NftPtrLib { web3: web3 }
}

pub fn make_nft_ptr_lib_localhost() -> NftPtrLib<web3::transports::Http> {
    let transport = web3::transports::Http::new("http://127.0.0.1:7545").unwrap();
    let web3 = web3::Web3::new(transport);
    NftPtrLib { web3: web3 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

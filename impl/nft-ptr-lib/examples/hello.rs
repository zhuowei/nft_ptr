use log::info;
use web3::types::{Address, U256};
#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Hello!");
    let mut lib = nft_ptr_lib::make_nft_ptr_lib_localhost();
    lib.initialize().await;
    //lib.mint_token(lib.account, U256::from(0x41414141), "Example")
    //    .await;
}

use log::info;
#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Hello!");
    let mut lib = nft_ptr_lib::make_nft_ptr_lib_localhost();
    lib.initialize().await;
}

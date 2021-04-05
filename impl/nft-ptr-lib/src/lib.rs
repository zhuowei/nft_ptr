use log::info;
use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;
use web3::api::Web3;
use web3::contract::Contract;
use web3::types::{Address, U256};

const NUM_CONFIRMATIONS: usize = 0;
const TOKEN_BASE_URI: &str = "https://nft-ptr.notnow.dev/?";

pub struct NftPtrLib<T: web3::Transport> {
    web3: Web3<T>,
    pub account: Address,
    token_contract: Option<Contract<T>>,
    instance_to_contract: HashMap<u64, Contract<T>>,
    num_confirmations: usize,
    network_id: u32,
}

impl<T: web3::Transport> NftPtrLib<T> {
    pub fn new(transport: T) -> NftPtrLib<T> {
        let web3 = web3::Web3::new(transport);
        let num_confirmations = std::env::var("NFT_PTR_NUM_CONFIRMATIONS")
            .map(|a| a.parse::<usize>().unwrap())
            .unwrap_or(NUM_CONFIRMATIONS);
        NftPtrLib {
            web3: web3,
            account: Address::zero(),
            token_contract: None,
            instance_to_contract: HashMap::new(),
            num_confirmations,
            network_id: 0,
        }
    }
    pub async fn initialize(&mut self) {
        self.check_not_prod().await;
        self.account = self.web3.eth().accounts().await.unwrap()[0];
        info!("Account: {:#x}", self.account);
        if self.is_goerli() {
            info!("https://goerli.etherscan.io/address/{:#x}", self.account);
        }
        info!("Deploying NFT contract!");
        self.deploy_token_contract().await;
        info!(
            "Token contract deployed at {:#x}",
            self.token_contract.as_ref().unwrap().address()
        );
        if self.is_goerli() {
            info!(
                "https://goerli.etherscan.io/token/{:#x}",
                self.token_contract.as_ref().unwrap().address()
            );
        }
    }
    async fn check_not_prod(&mut self) {
        let version = self.web3.net().version().await.unwrap();
        info!("Connected to network id {}", version);
        if version == "1" {
            panic!("Cowardly refusing to run on mainnet and waste real \"money\"");
        }
        self.network_id = version.parse::<u32>().unwrap();
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
        .confirmations(self.num_confirmations)
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

    fn mem_address_to_owner_contract_address(&self, a: u64) -> Address {
        if self.instance_to_contract.contains_key(&a) {
            return self.instance_to_contract[&a].address();
        }
        return self.account;
    }

    pub async fn move_token(
        &mut self,
        owner_address: u64,
        previous_owner_address: u64,
        value: u64,
        caller_pc: u64,
        object_type: &str,
    ) {
        let caller_pc_lineinfo = string_for_pc_addr(caller_pc);
        let caller_pc_backtrace_str = format!("{:x} {}", owner_address, caller_pc_lineinfo,);
        let object_type_demangled = demangle_cpp(object_type);
        let token_uri = format!("{:x} {}", value, object_type);
        let token_uri_encoded =
            percent_encoding::utf8_percent_encode(&token_uri, percent_encoding::NON_ALPHANUMERIC)
                .to_string();
        let owner_contract = self.mem_address_to_owner_contract_address(owner_address);
        let previous_owner_contract =
            self.mem_address_to_owner_contract_address(previous_owner_address);
        // TODO(zhuowei): figure out what to do with the caller_pc
        info!(
            "Transferring {:#x} ({}) to {:#x} ({:#x}) from {:#x} ({:#x}) at PC={:#x} ({})",
            value,
            object_type_demangled,
            owner_address,
            owner_contract,
            previous_owner_address,
            previous_owner_contract,
            caller_pc,
            caller_pc_lineinfo,
        );
        let transaction = self
            .token_contract
            .as_ref()
            .unwrap()
            .call_with_confirmations(
                "mintOrMove",
                (
                    owner_contract,
                    previous_owner_contract,
                    U256::from(value),
                    token_uri_encoded,
                    caller_pc_backtrace_str,
                ),
                self.account,
                web3::contract::Options::with(|opt| {
                    opt.gas = Some(1_000_000.into());
                }),
                self.num_confirmations,
            )
            .await
            .unwrap();
        info!("Transaction: {:#x}", transaction.transaction_hash);
        if self.is_goerli() {
            info!(
                "https://testnets.opensea.io/assets/goerli/{:#x}/{:#x}",
                self.token_contract.as_ref().unwrap().address(),
                value
            )
        }
    }
    pub async fn ptr_initialize(
        &mut self,
        owner_address: u64,
        caller_pc: u64,
        ptr_object_type: &str,
    ) {
        // rust-web3/examples/contract.rs
        // TODO(zhuowei): understand this
        let name = format!(
            "{:x} {} {}",
            owner_address,
            demangle_cpp(ptr_object_type),
            string_for_pc_addr(caller_pc),
        );
        info!("Deploying contract for nft_ptr {}", name);
        let my_account = self.account;
        let bytecode = include_str!("../../../contracts/out/NftPtrOwner.code");
        let contract = Contract::deploy(
            self.web3.eth(),
            include_bytes!("../../../contracts/out/NftPtrOwner.json"),
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
                // see NftPtrOwner.sol's constructor
                /*name*/
                name.to_owned(),
            ),
            my_account,
        )
        .await
        .unwrap();
        info!(
            "Deployed contract for nft_ptr {} at {:#x}",
            name,
            contract.address()
        );
        if self.is_goerli() {
            info!(
                "https://goerli.etherscan.io/token/{:#x}",
                contract.address()
            );
        }
        self.instance_to_contract.insert(owner_address, contract);
    }

    pub async fn ptr_destroy(&mut self, owner_address: u64) {
        // Don't actually destroy the contract so we can inspect later
        // TODO(zhuowei): actually destroy this pointer?
        self.instance_to_contract.remove(&owner_address);
    }
    fn is_goerli(&self) -> bool {
        self.network_id == 5
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

pub type NftPtrLibTransport =
    web3::transports::Either<web3::transports::Http, web3::transports::Ipc>;

pub async fn make_nft_ptr_lib() -> NftPtrLib<NftPtrLibTransport> {
    let ipc_path = std::env::var("NFT_PTR_IPC");
    let transport = if ipc_path.is_ok() {
        NftPtrLibTransport::Right(web3::transports::Ipc::new(ipc_path.unwrap()).await.unwrap())
    } else {
        NftPtrLibTransport::Left(
            web3::transports::Http::new(
                &std::env::var("NFT_PTR_HTTP").unwrap_or("http://127.0.0.1:7545".to_string()),
            )
            .unwrap(),
        )
    };
    NftPtrLib::new(transport)
}

fn string_for_pc_addr(pc_addr: u64) -> String {
    let mut outstr: Option<String> = None;
    let mut once: bool = false;
    backtrace::resolve(pc_addr as _, |symbol| {
        if once || symbol.name().is_none() {
            return;
        }
        once = true;
        if symbol.filename().is_some() && symbol.lineno().is_some() {
            let s = format!(
                "{} ({}:{})",
                demangle_cpp(symbol.name().unwrap().as_str().unwrap()),
                symbol
                    .filename()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy(),
                symbol.lineno().unwrap()
            );
            outstr = Some(s);
        } else {
            outstr = Some(demangle_cpp(symbol.name().unwrap().as_str().unwrap()));
        }
    });
    if !once {
        return format!("{:x}", pc_addr);
    }
    return outstr.unwrap();
}

fn demangle_cpp(typename: &str) -> String {
    // I could just call abi::__cxx_demangle in the C++, but lol WRITE IT IN RUST
    let demangled = cpp_demangle::Symbol::new(typename);
    if demangled.is_ok() {
        return demangled.unwrap().to_string();
    }
    return typename.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn demangle_cpp_example() {
        assert_eq!(demangle_cpp("P3Cow"), "Cow*");
    }
}

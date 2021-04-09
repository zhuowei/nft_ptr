C++ `std::unique_ptr` smart pointer replacement that ensures uniqueness by representing each object as an NFT on the Ethereum Blockchain.

# What works

- Deploying ERC721 smart contract on program start
- Call smart contract to create token when a pointer is transferred into an `nft_ptr`
- Tested on local chain using Ganache
- Create smart contract for each `nft_ptr` instance
- Transfer token when pointer moved between `nft_ptr`s

# Building

All instructions tested on macOS 11.2.1.

You need:
- [Rustup](https://www.rust-lang.org)
- [npm](https://nodejs.org/)
- [Truffle](https://www.trufflesuite.com/truffle)
- [OpenSSL](https://github.com/sfackler/rust-openssl/blob/140ec193d5ad1ab91321069ccb2f86be947fafee/openssl/src/lib.rs#L30) (`brew install openssl@1.1`)
- and a C++17 compiler (Tested with Clang from Xcode 14.3).

```
cd contracts
npm install
truffle compile
./dumpbytecode
cd ../impl
rustup override set nightly
cargo build
cd ../example
./build.sh
```

# Testing (local blockchain)

Download and run [Ganache](https://www.trufflesuite.com/ganache) to setup a local blockchain. Then, run

```
cd example
RUST_BACKTRACE=1 RUST_LOG=info ./example
```

# Testing (Görli testnet)

Create a new wallet and get some Görli test ethers from the [Görli faucet](https://faucet.goerli.mudit.blog).

Download Geth and start a lite node connected to the Görli testnet:

```
./geth --goerli --syncmode light
```

Stop Geth and import your testnet wallet:

```
cp ~/Downloads/MewWallet.keystore ~/Library/Ethereum/goerli/keystore/
```

Restart Geth and unlock your testnet wallet:

```
./geth --goerli --unlock 0x<address> --syncmode light
```

Enter your password, then hit Enter. It should say

```
Unlocked account                         address=0x<address>
```

Finally run with IPC transport:

```
cd example
RUST_BACKTRACE=1 RUST_LOG=info NFT_PTR_IPC=$HOME/Library/Ethereum/goerli/geth.ipc ./example
```

# Thanks

C++ `std::unique_ptr` smart pointer replacement that ensures uniqueness by representing each object as an NFT on the Ethereum Blockchain.

# What works

- Deploying ERC721 smart contract on program start
- Call smart contract to create token when a pointer is transferred into an `nft_ptr`
- Tested on local chain using Ganache

# What's being worked on

- Create smart contract for each `nft_ptr` instance
- Transfer token when pointer moved between `nft_ptr`s

# Building

You need Rustup, Truffle, Ganache, npm, and a C++17 compiler.

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
RUST_BACKTRACE=1 RUST_LOG=info ./example
```

# Thanks
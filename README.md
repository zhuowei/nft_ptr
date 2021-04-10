C++ `std::unique_ptr` that represents each object as an NFT on the Ethereum Blockchain.

[![Build nft_ptr](https://github.com/zhuowei/nft_ptr/actions/workflows/build.yml/badge.svg)](https://github.com/zhuowei/nft_ptr/actions/workflows/build.yml)

# Example: moving between two `nft_ptr`s

```cpp
  auto ptr1 = make_nft<Cow>();
  nft_ptr<Animal> ptr2;

  ptr2 = std::move(ptr1);
```

This [transfers](https://goerli.etherscan.io/tx/0xcbe06fdd54bd9d221993c875022fe2960128874811a25075d692cc638a28f290) the Non-Fungible Token [0x7faa4bc09c90](https://testnets.opensea.io/assets/goerli/0x90eaf0ab2c6455a9b794f9dcf97839fa25b4ce2d/0x7faa4bc09c90), representing the `Cow`'s memory address, from `ptr1` ([OpenSea](https://testnets.opensea.io/accounts/0x9ed6006c6f3bb20737bdbe88cc6aa0de00597fef?tab=activity), [Etherscan](https://goerli.etherscan.io/address/0x9ed6006c6f3bb20737bdbe88cc6aa0de00597fef#tokentxnsErc721)) to `ptr2` ([OpenSea](https://testnets.opensea.io/accounts/0x1564b0a7c258fc88a96aa9fe1c513101883abb13?tab=activity), [Etherscan](https://goerli.etherscan.io/address/0x1564b0a7c258fc88a96aa9fe1c513101883abb13#tokentxnsErc721)).

![screenshot of OpenSea Trading History showing token transfer](example/logs/opensea_transfer_ptr1_ptr2.png)

```
[2021-04-09T01:59:48Z INFO  nft_ptr_lib] Transferring 0x7faa4bc09c90 (Cow) to 0x7ffee35a7890 (0x1564b0a7c258fc88a96aa9fe1c513101883abb13) from 0x7ffee35a78a8 (0x9ed6006c6f3bb20737bdbe88cc6aa0de00597fef) at PC=0x10c65a946 (main (example.cpp:33))
[2021-04-09T02:00:15Z INFO  nft_ptr_lib] Transaction: 0xcbe06fdd54bd9d221993c875022fe2960128874811a25075d692cc638a28f290
[2021-04-09T02:00:15Z INFO  nft_ptr_lib] https://testnets.opensea.io/assets/goerli/0x90eaf0ab2c6455a9b794f9dcf97839fa25b4ce2d/0x7faa4bc09c90
```

# Example: constructing an `nft_ptr` and minting an NFT

```cpp
  auto ptr1 = make_nft<Cow>();
```

This:
- initializes the `nft_ptr` runtime
- creates the first `nft_ptr<Cow>`
- transfers ownership of the newly created `Cow*` to the `nft_ptr`

First, it creates [an ERC-721 smart contract](https://goerli.etherscan.io/token/0x90eaf0ab2c6455a9b794f9dcf97839fa25b4ce2d) that represents each memory address as a Non-Fungible Token.

```
[2021-04-09T01:57:48Z INFO  nft_ptr_lib] Connected to network id 5
[2021-04-09T01:57:48Z INFO  nft_ptr_lib] Account: 0xd54b39c6bb7774aba2be4b49dc2667332b737909
[2021-04-09T01:57:48Z INFO  nft_ptr_lib] https://goerli.etherscan.io/address/0xd54b39c6bb7774aba2be4b49dc2667332b737909
[2021-04-09T01:57:48Z INFO  nft_ptr_lib] Deploying NFT contract!
[2021-04-09T01:58:18Z INFO  nft_ptr_lib] Token contract deployed at 0x90eaf0ab2c6455a9b794f9dcf97839fa25b4ce2d
[2021-04-09T01:58:18Z INFO  nft_ptr_lib] https://goerli.etherscan.io/token/0x90eaf0ab2c6455a9b794f9dcf97839fa25b4ce2d
```

Next, it creates [another smart contract](https://goerli.etherscan.io/token/0x9ed6006c6f3bb20737bdbe88cc6aa0de00597fef), that represents the `nft_ptr<Cow>` instance which can own NftPtr tokens:

```
[2021-04-09T01:58:18Z INFO  nft_ptr_lib] Deploying contract for nft_ptr 7ffee35a78a8 Cow main (example.cpp:25)
[2021-04-09T01:58:48Z INFO  nft_ptr_lib] Deployed contract for nft_ptr 7ffee35a78a8 Cow main (example.cpp:25) at 0x9ed6006c6f3bb20737bdbe88cc6aa0de00597fef
[2021-04-09T01:58:48Z INFO  nft_ptr_lib] https://goerli.etherscan.io/token/0x9ed6006c6f3bb20737bdbe88cc6aa0de00597fef
```

Finally, it calls `new Cow()`, and [mints an NFT](https://testnets.opensea.io/assets/goerli/0x90eaf0ab2c6455a9b794f9dcf97839fa25b4ce2d/0x7faa4bc09c90) for this memory address, owned by the new `nft_ptr<Cow>`.

```
[2021-04-09T01:58:48Z INFO  nft_ptr_lib] Transferring 0x7faa4bc09c90 (Cow) to 0x7ffee35a78a8 (0x9ed6006c6f3bb20737bdbe88cc6aa0de00597fef) from 0x0 (0xd54b39c6bb7774aba2be4b49dc2667332b737909) at PC=0x10c65a76f (main (example.cpp:25))
[2021-04-09T01:59:18Z INFO  nft_ptr_lib] Transaction: 0x0a148cee1abe8d4b5721996ea3a107c87b526ded155dc2e3895f1f42983bd2e8
[2021-04-09T01:59:18Z INFO  nft_ptr_lib] https://testnets.opensea.io/assets/goerli/0x90eaf0ab2c6455a9b794f9dcf97839fa25b4ce2d/0x7faa4bc09c90
```

# More examples

A full example program can be found at [example/example.cpp](example/example.cpp), along with a sample of its [output](example/logs/log_goerli_hosted.txt) when run.

A longer example, which shows using `nft_ptr` with function calls and STL containers, can be found at [example/long_example.cpp](example/long_example.cpp) along with its [output](example/logs/log_long_example_goerli.txt).

# Performance

`nft_ptr` has negligible performance overhead compared to `std::unique_ptr`, as shown by this benchmark on [our example program](example/example.cpp):

| Implementation    | Runtime       |
|-------------------|:-------------:|
| `std::unique_ptr` | 0.005 seconds |
| `nft_ptr`         | 3 minutes     |

# What works

- Deploying ERC721 smart contract on program start
- Call smart contract to create token when a pointer is transferred into an `nft_ptr`
- Tested on local chain using Ganache
- Create smart contract for each `nft_ptr` instance
- Transfer token when pointer moved between `nft_ptr`s

For more information, please read our [white paper](white_paper.pdf).

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

# Sponsor development

For a limited time, you can buy any Git commit from this repository as a Non-Fungible Token on my Content-First Multimedia Proof-of-Authority revision-controlled realtime collaborative private enterprise [Blockchain](https://docs.google.com/document/d/1d03A_-BAgwFZgmHh3TzEbGsvQ33albI9WT3fLa9gjtQ/edit) (a shared Google Doc).
#!/bin/bash
set -e -x

# setup
sudo npm install -g truffle

# compile
cd contracts
npm install
truffle compile
./dumpbytecode
cd ../impl
rustup override set nightly
cargo build
cd ../example
./build.sh
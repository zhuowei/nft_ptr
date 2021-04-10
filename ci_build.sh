#!/bin/bash
set -e -x

# compile
cd contracts
npm install
npx truffle compile
./dumpbytecode
cd ../impl
rustup override set nightly
cargo build
cd ../example
./build.sh
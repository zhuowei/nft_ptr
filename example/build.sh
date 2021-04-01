#!/bin/sh
exec clang++ -g -std=c++17 -Wall -o example example.cpp ../impl/target/debug/libnft_ptr.dylib

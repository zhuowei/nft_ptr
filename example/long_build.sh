#!/bin/sh
exec clang++ -g -std=c++17 -Wall -o long_example long_example.cpp ../impl/target/debug/libnft_ptr.dylib

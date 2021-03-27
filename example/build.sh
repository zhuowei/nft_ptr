#!/bin/sh
exec clang++ -std=c++17 -Wall -o example example.cpp ../impl/target/debug/libnft_ptr.a

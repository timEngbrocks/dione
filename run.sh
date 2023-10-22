#!/bin/sh

cargo build

RUST_BACKTRACE=1 ./target/debug/dione -c Main.class -j ./jdk
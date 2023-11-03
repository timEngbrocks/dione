#!/bin/sh

cargo build

EXIT=$(echo $?)

if [ $EXIT -ne 0 ]; then
	exit $EXIT
fi

RUST_BACKTRACE=1 ./target/debug/dione -c Main -j ./jdk
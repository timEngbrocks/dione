#!/bin/sh

cargo build

EXIT=$(echo $?)

if [ $EXIT -ne 0 ]; then
	exit $EXIT
fi

gdb --args ./target/debug/dione -c Main.class -j ./jdk
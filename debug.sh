#!/bin/sh

cargo build

gdb --args ./target/debug/dione -c Main.class -j ./jdk
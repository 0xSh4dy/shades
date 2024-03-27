#!/bin/bash
cargo run tests/stage2/test1.rh
llc-16 -filetype=obj /tmp/main.ll -o /tmp/main.o
clang -no-pie /tmp/main.o -o /tmp/main
/tmp/main
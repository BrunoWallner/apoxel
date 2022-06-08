#!/bin/sh

# all x86_64

# linux
cargo b --release --target x86_64-unknown-linux-gnu
mv ./target/x86_64-unknown-linux-gnu/release/librust.so ../../Godot/Bin/lib.so

# windows
cargo b --release --target x86_64-pc-windows-gnu
mv ./target/x86_64-pc-windows-gnu/release/rust.dll ../../Godot/Bin/lib.dll

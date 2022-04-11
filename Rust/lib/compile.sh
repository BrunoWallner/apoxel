#!/bin/bash

cargo b --release
mv ./target/release/librust.so ../../Godot/Bin/

#!/bin/bash

cargo b --release
sudo mv ./target/release/vox_to_structure /usr/bin/
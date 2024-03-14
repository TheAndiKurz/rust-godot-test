#!/bin/bash

# make the target directory
mkdir -p target/debug

# build the rust library
cargo build --manifest-path rust/Cargo.toml


# export the game
godot godot/project.godot --export-debug "Linux/X11" ../target/debug/game

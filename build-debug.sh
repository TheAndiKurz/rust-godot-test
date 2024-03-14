#!/bin/bash

# make the target directory
mkdir -p target/debug

# export the game
godot godot/project.godot --export-debug "Linux/X11" ../target/debug/game

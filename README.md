# Rust Godot Binding Test Project

This project is here to test how the bindings between rust and godot work using [gdext](https://github.com/godot-rust/gdext).

Also this project is to see how smooth the working between rust and godot is.

## Setup

For the setup first follow the steps [here](https://godot-rust.github.io/book/intro/setup.html).

If you are done with that you can go into the rust directory and run:
```
cargo build
```

Then run godot and select the `godot/project.godot` file to open the project.

You can now run the game and it should show you a rotating city with shadows.

## Running the game without godot editor

This is only available to use if you are on linux for now!!!

First you need godot and rust you can get how to install them from [here](https://godot-rust.github.io/book/intro/setup.html).

Next you can build the hole game and export it to an executable with:
```
bash build-debug.sh
```

If you executed that script you will get a target folder with the game inside.
To execute it from command line you can use:
```
./target/debug/game
```
or
```
./target/debug/game
```


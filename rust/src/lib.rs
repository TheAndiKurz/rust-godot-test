mod rotating_node3d;
mod frame_rate_text;
mod zoomable_camera;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


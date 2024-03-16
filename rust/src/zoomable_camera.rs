use godot::{engine::{global::MouseButton, *}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Camera3D)]
pub struct ZoomableCamera {
    #[export]
    zoom_factor: f32,

    base: Base<Camera3D>
}

#[godot_api]
impl ICamera3D for ZoomableCamera {
    fn init(base: Base<Camera3D>) -> Self {
        ZoomableCamera {
            zoom_factor: 1.0,
            base
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        match event.try_cast::<InputEventMouseButton>() {
            Ok(event) => {
                if event.is_pressed() {
                    let direction = self.base().get_global_transform().basis.col_c();
                    let zoom_direction = direction * self.zoom_factor;

                    let position = self.base().get_position();

                    if event.get_button_index() == MouseButton::WHEEL_UP {
                        self.base_mut().set_position(position + zoom_direction);
                    } else if event.get_button_index() == MouseButton::WHEEL_DOWN {
                        self.base_mut().set_position(position - zoom_direction);
                    }
                }
            }
            _ => {}
        }
    }
}



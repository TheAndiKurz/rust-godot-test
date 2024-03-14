use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RotatingNode3D {
    #[export]
    angular_speed: f64,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for RotatingNode3D {
    fn init(base: Base<Node3D>) -> Self {
        println!("RotatingNode3D init");

        RotatingNode3D {
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate_y(radians);
    }
}

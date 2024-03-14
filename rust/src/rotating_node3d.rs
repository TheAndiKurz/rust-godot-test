use godot::prelude::*;

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum Direction {
    X,
    Y,
    Z
}


#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RotatingNode3D {
    #[export]
    angular_speed: f64,
    base: Base<Node3D>,

    #[export]
    direction: Direction,
}

#[godot_api]
impl INode3D for RotatingNode3D {
    fn init(base: Base<Node3D>) -> Self {
        println!("RotatingNode3D init");

        RotatingNode3D {
            direction: Direction::Y,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;

        let axis = match self.direction {
            Direction::X => Vector3::new(1.0, 0.0, 0.0),
            Direction::Y => Vector3::new(0.0, 1.0, 0.0),
            Direction::Z => Vector3::new(0.0, 0.0, 1.0),
        };
        self.base_mut().rotate(axis, radians)
    }
}

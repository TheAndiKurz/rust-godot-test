use godot::{engine::{IRichTextLabel, RichTextLabel}, prelude::*};

#[derive(GodotClass)]
#[class(base=RichTextLabel)]
pub struct FrameRateText {
    base: Base<RichTextLabel>,
}

#[godot_api]
impl IRichTextLabel for FrameRateText {
    fn init(base: Base<RichTextLabel>) -> Self {
        FrameRateText { base }
    }

    fn process(&mut self, delta: f64) {
        let frame_rate = 1.0 / delta;

        let text = format!("{:.2} fps", frame_rate);
        self.base_mut().set_text(text.into());
    }
}



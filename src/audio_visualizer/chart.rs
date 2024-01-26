use iced::{Color, mouse, Rectangle, Renderer, Theme};
use iced::widget::canvas::{Frame, Geometry, Path, Program};
use crate::audio_visualizer::UiMessage;

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Program<UiMessage> for Circle {
    type State = ();

    fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry>{
        // We prepare a new `Frame`
        let mut frame = Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle = Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}
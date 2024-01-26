use iced::{Color, mouse, Point, Rectangle, Renderer, Theme};
use iced::widget::canvas::{Frame, Geometry, Path, Program, Stroke};
use crate::audio_visualizer::UiMessage;

#[derive(Debug)]
pub struct SineWave {
    pub amplitude: f32,
    pub frequency: f32,
}

impl Program<UiMessage> for SineWave {
    type State = ();

    fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let stroke = Stroke::default().with_color(Color::BLACK);
        let path = Path::new(|path| {
            for x in 0..bounds.width as i32 {
                let y = bounds.height / 2.0 + self.amplitude * f32::sin(self.frequency * x as f32);
                if x == 0 {
                    path.move_to(Point::new(x as f32, y));
                } else {
                    path.line_to(Point::new(x as f32, y));
                }
            }
        });

        frame.stroke(&path, stroke);

        vec![frame.into_geometry()]
    }
}

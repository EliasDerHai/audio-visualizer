use iced::widget::canvas::{self, Cursor, Frame, Geometry, Path, Stroke};
use iced::{Color, Point, Rectangle, Theme};
use std::time::{Duration, Instant};

use super::UiMessage;

pub struct State {
    phase_shift: f32,
    last_update: Instant,
}

impl Default for State {
    fn default() -> Self {
        State {
            phase_shift: 0.0,
            last_update: Instant::now(),
        }
    }
}

pub struct ChartCanvas;

impl canvas::Program<UiMessage> for ChartCanvas {
    type State = State;

    fn update(
        &self,
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (canvas::event::Status, Option<UiMessage>) {
        let now = Instant::now();
        if now.duration_since(_state.last_update) >= Duration::from_millis(16) {
            _state.phase_shift += 0.5;
            _state.last_update = now;
            (canvas::event::Status::Captured, None)
        } else {
            (canvas::event::Status::Ignored, None)
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        let stroke = Stroke::default().with_color(Color::WHITE);

        let path = Path::new(|path| {
            for x in 0..bounds.width as i32 {
                let y: f32 = (bounds.height / 2.0)
                    + 20.0 * f32::sin(((x as f32) + _state.phase_shift) * 0.1);
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

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> iced::mouse::Interaction {
        iced::mouse::Interaction::default()
    }
}

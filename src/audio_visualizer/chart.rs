use iced::widget::canvas::{self, Cursor, Frame, Geometry, Path, Stroke};
use iced::{Point, Rectangle, Theme};

use super::UiMessage;

pub struct ChartCanvas;

impl canvas::Program<UiMessage> for ChartCanvas {
    type State = (); 
    
    fn update(
        &self,
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (canvas::event::Status, Option<UiMessage>) {
        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        let stroke = Stroke::default();

        // simple line
        let path = Path::line(
            Point::new(0.0, bounds.height),
            Point::new(bounds.width, 0.0),
        );
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

mod audio_visualizer;

use audio_visualizer::AudioVisualizer;
use iced::{Application, Settings};

fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        size: (600, 600),
        ..Default::default()
    };

    let settings = Settings {
        window: window_settings,
        ..Settings::default()
    };

    AudioVisualizer::run(settings)
}

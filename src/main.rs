mod audio_visualizer;

use audio_visualizer::AudioVisualizer;
use iced::{Application, Settings};

fn main() -> iced::Result {
    AudioVisualizer::run(Settings::default())
}

mod audio_visualizer;

use audio_visualizer::AudioVisualizer;
use iced::{Application, Settings};

// TODO remove debug import  
use cpal::traits::{DeviceTrait, HostTrait};

fn main() -> iced::Result {
    list_output_devices();
    AudioVisualizer::run(Settings::default())
}

// TODO remove debug code  
fn list_output_devices() {
    let host = cpal::default_host();
    let output_devices = host.output_devices().unwrap();

    println!("Available output devices:");
    for (index, device) in output_devices.enumerate() {
        println!("{}. {}", index + 1, device.name().unwrap());
    }
}
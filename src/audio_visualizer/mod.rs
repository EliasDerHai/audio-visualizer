mod chart;

use iced::{
    widget::{Button, Column, Text, Canvas},
    Application, Command,
};
use native_dialog::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use crate::audio_visualizer::chart::Circle;


pub struct AudioVisualizer {
    file_path: Option<PathBuf>,
    audio_command_sender: mpsc::Sender<AudioCommand>,
}

#[derive(Debug, Clone)]
pub enum UiMessage {
    OpenPressed,
    PlayPressed,
    StopPressed,
}

#[derive(Debug, Clone)]
enum AudioCommand {
    Play(PathBuf),
    Stop,
}

impl Application for AudioVisualizer {
    type Executor = iced::executor::Default;
    type Message = UiMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (AudioVisualizer, Command<Self::Message>) {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            loop {
                if let Ok(command) = receiver.try_recv() {
                    process_audio_command(command, &sink);
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        (
            AudioVisualizer {
                file_path: None,
                audio_command_sender: sender,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Audio Visualizer")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            UiMessage::OpenPressed => {
                match FileDialog::new()
                    .add_filter("Audio Files", &["mp3"])
                    .show_open_single_file()
                {
                    Ok(Some(path)) => {
                        println!("File selected: {:?}", path.file_name());
                        self.file_path = Some(path);
                    }
                    Ok(None) => {
                        self.file_path = None;
                    }
                    Err(err) => {
                        println!("File dialog error: {:?}", err);
                        self.file_path = None;
                    }
                }
            }
            UiMessage::PlayPressed => {
                println!("Start pressed");
                if let Some(path) = self.file_path.clone() {
                    self.audio_command_sender
                        .send(AudioCommand::Play(path))
                        .unwrap();
                }
            }
            UiMessage::StopPressed => {
                println!("Stop pressed");
                self.audio_command_sender.send(AudioCommand::Stop).unwrap();
            }
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let file_name = self
            .file_path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
            .unwrap_or("-".to_string());

        let open_button = Button::new(Text::new("Open")).on_press(UiMessage::OpenPressed);
        let file_text = Text::new(file_name);
        let play_button = Button::new(Text::new("Play")).on_press(UiMessage::PlayPressed);
        let stop_button = Button::new(Text::new("Stop")).on_press(UiMessage::StopPressed);

        let canvas = Canvas::new(Circle { radius: 50.0 })
            .width(iced::Length::Fill);

        Column::new()
            .push(open_button)
            .push(file_text)
            .push(play_button)
            .push(stop_button)
            .push(canvas)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dark
    }
}

fn process_audio_command(command: AudioCommand, sink: &Sink) {
    match command {
        AudioCommand::Play(path) => {
            if let Ok(file) = File::open(path) {
                if let Ok(source) = Decoder::new(BufReader::new(file)) {
                    sink.append(source);
                }
            }
        }
        AudioCommand::Stop => {
            sink.stop();
        }
    }
}

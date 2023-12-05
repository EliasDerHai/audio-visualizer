use iced::{
    widget::{Button, Column, Text},
    Application, Command,
};
use native_dialog::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::path::PathBuf;

pub struct AudioVisualizer {
    file_path: Option<PathBuf>
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenPressed,
    PlayPressed,
}

impl Application for AudioVisualizer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: Self::Flags) -> (AudioVisualizer, Command<Self::Message>) {
        (
            AudioVisualizer {
                file_path: None
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Audio Visualizer")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Message::OpenPressed => {
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
            Message::PlayPressed => {
                println!("Start pressed");
                if let Some(path) = &self.file_path {
                    match File::open(path) {
                        Ok(file) => match Decoder::new(std::io::BufReader::new(file)) {
                            Ok(source) => {
                                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                                match Sink::try_new(&stream_handle) {
                                    Ok(sink) => {
                                        sink.append(source);
                                        sink.sleep_until_end();
                                    }
                                    Err(e) => println!("Error creating sink: {:?}", e),
                                }
                            }
                            Err(e) => println!("Error decoding audio: {:?}", e),
                        },
                        Err(e) => println!("Error opening file: {:?}", e),
                    }
                } 
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

        let open_button = Button::new(Text::new("Open")).on_press(Message::OpenPressed);
        let file_text = Text::new(file_name);
        let play_button = Button::new(Text::new("Play")).on_press(Message::PlayPressed);

        Column::new()
            .push(open_button)
            .push(file_text)
            .push(play_button)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dark
    }
}

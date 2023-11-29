use core::time::Duration;
use iced::{
    widget::{Button, Column, Text},
    Application, Command,
};
use native_dialog::FileDialog;
use rodio::source::SineWave;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::path::PathBuf;

pub struct AudioVisualizer {
    file_path: Option<PathBuf>,
    audio_output: Option<OutputStreamHandle>,
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
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let source = SineWave::new(440.0)
            .take_duration(Duration::from_secs_f32(3.0))
            .amplify(0.20);
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();

        (
            AudioVisualizer {
                file_path: None,
                audio_output: Some(stream_handle),
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
                        Ok(file) => {
                            match Decoder::new(std::io::BufReader::new(file)) {
                                Ok(source) => {
                                    // Create an OutputStreamHandle inline
                                    let (_stream, stream_handle) =
                                        OutputStream::try_default().unwrap();

                                    // Create a Sink and play the audio
                                    let sink = Sink::try_new(&stream_handle).unwrap();
                                    sink.append(source);

                                    // Block the current thread until all queued sounds have finished playing
                                    sink.sleep_until_end();
                                }
                                Err(e) => println!("Error decoding audio: {:?}", e),
                            }
                        }
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

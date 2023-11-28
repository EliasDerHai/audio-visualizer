use iced::widget::Text;
use iced::{Application, Command};
use iced::widget::Button;
use native_dialog::FileDialog;
use std::path::PathBuf;

pub struct AudioVisualizer{
    file_path: Option<PathBuf>
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenPressed,
    // Other messages...
 }

impl Application for AudioVisualizer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    
    fn new(_flags: Self::Flags) -> (AudioVisualizer, Command<Self::Message>) {
        (AudioVisualizer{ file_path: None}, Command::none())
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
                    },
                    Ok(None) => {
                        self.file_path = None;
                    },
                    Err(err) => {
                        println!("File dialog error: {:?}", err);
                        self.file_path = None;
                    }
                }
            },
        }
     
        Command::none()    
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Button::new(Text::new("Open")).on_press(Message::OpenPressed).into()
        // Text::new(greeting).into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dark
    }
}
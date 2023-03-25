#![windows_subsystem = "windows"]

use std::path::PathBuf;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, text};
use iced::{window, Element, Length, Sandbox, Settings, Theme};
use iced_native::widget::{button, container, row, svg, Column, Row, Space};
use iced_native::{row, Alignment};

struct FileShuffler {
    current_directory: PathBuf,
    file_list: Vec<String>,
    opened_files: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    ClickedNext,
    ClickedClear,
    ClickedScan,
}

impl Sandbox for FileShuffler {
    type Message = Message;

    fn new() -> Self {
        FileShuffler {
            current_directory: PathBuf::new(),
            file_list: vec![],
            opened_files: vec![],
        }
    }

    fn title(&self) -> String {
        String::from("Jini File Shuffler")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ClickedNext => {}
            Message::ClickedClear => {}
            Message::ClickedScan => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let clear_button = button("Clear Queue");
        let next_button = button("Next File");
        let scan_button = button("Refresh Queue");
        let choose_folder_btn = button("Choose Folder");

        let main_column = Column::new().align_items(Alignment::Center);
        let directory_text = container(text("Choose a Folder"))
            .align_y(Vertical::Bottom)
            .height(Length::Fill);

        let button_row = container(
            Row::new()
                .spacing(15.0)
                .align_items(Alignment::Center)
                .push(clear_button)
                .push(next_button)
                .push(scan_button)
                .push(choose_folder_btn),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Bottom)
        .width(Length::Fill)
        .height(Length::Fill);

        main_column
            .push(directory_text)
            .push(Space::new(10.0, 10.0))
            .push(button_row)
            .push(Space::new(10.0, 10.0))
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() {
    println!("Jini File Shuffler");
    let settings = Settings {
        window: window::Settings {
            size: (500, 200),
            resizable: true,
            decorations: true,

            ..Default::default()
        },
        ..Default::default()
    };
    FileShuffler::run(settings).unwrap();
}
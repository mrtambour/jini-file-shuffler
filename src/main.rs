#![windows_subsystem = "windows"]

use std::path::PathBuf;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, text};
use iced::{window, Element, Length, Sandbox, Settings, Theme};
use iced_native::widget::{button, container, row, svg, Column, Row, Space};
use iced_native::{row, Alignment};

use crate::files::{choose_folder, scan_folder};

mod files;

struct FileShuffler {
    current_directory: PathBuf,
    file_list: Vec<String>,
    opened_files: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    ClickedNext,
    ClickedClear,
    ClickedRefresh,
    ClickedChooseFolder,
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
            Message::ClickedClear => {
                self.file_list.clear();
                self.current_directory.clear()
            }
            Message::ClickedRefresh => {
                if self.file_list.is_empty() {
                } else {
                    match scan_folder(&self.current_directory) {
                        Ok(directory) => {
                            self.file_list.clear();

                            for file in directory {
                                self.file_list
                                    .push(file.unwrap().file_name().to_str().unwrap().to_string())
                            }
                        }
                        Err(error) => {
                            println!("error scanning folder")
                        }
                    }
                }
            }

            Message::ClickedChooseFolder => match choose_folder() {
                Some(path) => {
                    self.current_directory = path;
                    match scan_folder(&self.current_directory) {
                        Ok(directory) => {
                            self.file_list.clear();

                            for file in directory {
                                self.file_list
                                    .push(file.unwrap().file_name().to_str().unwrap().to_string())
                            }
                        }
                        Err(error) => {
                            println!("error scanning folder")
                        }
                    }
                }
                None => {
                    println!("error selecting folder")
                }
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let clear_button = button("Clear Queue").on_press(Message::ClickedClear);
        let next_button = button("Next File");
        let scan_button = button("Refresh Queue").on_press(Message::ClickedRefresh);
        let choose_folder_btn = button("Choose Folder").on_press(Message::ClickedChooseFolder);

        let main_column = Column::new().align_items(Alignment::Center);
        let directory_text = container(text(self.current_directory.to_str().unwrap()))
            .align_y(Vertical::Bottom)
            .height(Length::Fill);
        let file_count_text = container(text(format!("File Count: {}", &self.file_list.len())))
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
            .push(file_count_text)
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

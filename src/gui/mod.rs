use iced::Alignment;
use iced::widget::image::Handle;
use iced::widget::{button, checkbox, column, container, row, scrollable, text};
use iced::{Element, Length, Task};

use image::DynamicImage;

use crate::file_ops::get_all_image_paths;
use crate::image_processing::Image;
use std::path::PathBuf;

pub mod dialogs;
pub mod widgets;
pub const DEFAULT_WINDOW_SIZE: (u32, u32) = (1200, 800);

#[derive(Default, Debug)]
pub struct ApplicationState {
    input_path: PathBuf,
    output_path: PathBuf,
    is_classified_dataset: bool,
    image_list: Vec<Image>,
    current_image_index: usize,
    message: String,
    current_image_texture: Option<Handle>,
    is_drawing_cropbox: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    Nothing,
    OpenInputFileDialog,
    OpenOutputFileDialog,
    InputFolderSelected(Result<PathBuf, String>),
    OutputFolderSelected(Result<PathBuf, String>),
    ImagesLoaded(Vec<PathBuf>),
    ImageSelected(usize),
    // New messages for async image loading
    ImageLoadedForDisplay {
        index: usize,
        image: DynamicImage,
        texture: iced::widget::image::Handle,
    },
    ImageLoadFailed {
        index: usize,
        error: String,
    },
    NextImage,
    PreviousImage,
    SetDataSetType(bool),
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            input_path: PathBuf::new(),
            output_path: PathBuf::new(),
            is_classified_dataset: true,
            image_list: vec![],
            current_image_index: 0,
            current_image_texture: None,
            is_drawing_cropbox: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Nothing => Task::none(),
            Message::OpenInputFileDialog => {
                println!("This is Running ?? ");
                Task::perform(dialogs::pick_folder(), |result| {
                    Message::InputFolderSelected(result)
                })
            }
            Message::OpenOutputFileDialog => {
                println!("Opening Output  Dialog");
                Task::perform(dialogs::pick_folder(), |result| {
                    Message::OutputFolderSelected(result)
                })
            }
            Message::InputFolderSelected(path_buf) => {
                match path_buf {
                    Ok(path) => {
                        self.input_path = path.clone();
                        let image_paths = get_all_image_paths(path);

                        println!("done getting image paths");
                        return Task::done(Message::ImagesLoaded(image_paths));
                    }
                    Err(_) => self.message = String::from("Error picking file"),
                };
                println!("No path");
                Task::none()
            }
            Message::OutputFolderSelected(path_buf) => {
                match path_buf {
                    Ok(path) => {
                        self.output_path = path.clone();
                    }
                    Err(_) => self.message = String::from("Error picking file"),
                };
                Task::none()
            }
            Message::ImagesLoaded(path_bufs) => {
                let mut images = vec![];
                println!("Starting to load images");
                for image_path in path_bufs {
                    let image = Image::new(image_path.clone());
                    images.push(image);
                    println!("Loaded {:?}", image_path);
                }
                println!("Loaded Images");
                self.image_list = images;
                Task::none()
            }
            Message::ImageSelected(index) => {
                self.current_image_index = index;
                Task::none()
            }
            Message::ImageLoadedForDisplay {
                index,
                image,
                texture,
            } => todo!(),
            Message::ImageLoadFailed { index, error } => todo!(),
            Message::NextImage => todo!(),
            Message::PreviousImage => todo!(),
            Message::SetDataSetType(is_checked) => {
                self.is_classified_dataset = is_checked;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let top_bar = row![
            button("Select Input Dir").on_press(Message::OpenInputFileDialog),
            text(self.input_path.to_string_lossy()).width(Length::FillPortion(1)),
            button("Select Output Dir").on_press(Message::OpenOutputFileDialog),
            text(self.output_path.to_string_lossy()).width(Length::FillPortion(1)),
            checkbox("Is Classified Dataset", self.is_classified_dataset,)
                .on_toggle(Message::SetDataSetType)
                .spacing(5),
        ]
        .padding(10)
        .spacing(10)
        .align_y(Alignment::Center)
        .width(Length::Fill);

        // Image List (Left Side)
        let image_list_items: Vec<Element<Message>> = self
            .image_list
            .iter()
            .enumerate()
            .map(|(index, image)| {
                let file_name = image.get_image_name();
                button(text(file_name))
                    .on_press(Message::ImageSelected(index))
                    .width(Length::Fill)
                    .into()
            })
            .collect();

        let image_list_scrollable =
            scrollable(column(image_list_items).spacing(5).width(Length::Fill))
                .width(Length::FillPortion(1))
                .height(Length::Fill);
        // let current_image = self.image_list.get(self.current_image_index);
        //
        // let current_image_name = match current_image {
        //     Some(image) => image.get_image_name(),
        //     None => String::from("None"),
        // };

        let main_image_area: Element<Message> = match self.image_list.get(self.current_image_index)
        {
            Some(image) => {
                let handle = image.get_iced_image_handle();
                let image_widget: iced::widget::image::Image<Handle> = iced::widget::image(handle);

                container(image_widget)
                    .width(Length::FillPortion(5))
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .into()
            }
            None => container(
                text("No image selected")
                    .size(20)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(Alignment::Center)
                    .align_x(Alignment::Center),
            )
            .width(Length::FillPortion(5))
            .height(Length::Fill)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
            .into(),
        };
        let content = row![image_list_scrollable, main_image_area,]
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill);

        column![top_bar, content,].padding(0).into()
    }
}

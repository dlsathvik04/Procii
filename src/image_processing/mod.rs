use std::path::PathBuf;

use iced::widget::image::Handle;
use image::DynamicImage;

#[derive(Debug)]
pub struct CropBox {
    x: u32,
    y: u32,
    height: u32,
    width: u32,
}

#[derive(Debug)]
pub struct Image {
    path: PathBuf,
    crop_boxes: Vec<CropBox>,
}

impl Image {
    pub fn new(path: PathBuf) -> Self {
        let crop_boxes = vec![];
        Self { path, crop_boxes }
    }

    pub fn get_image_data(&self) -> DynamicImage {
        image::open(&self.path).expect("Cant open the image")
    }

    pub fn add_crop_box(&mut self, crop_box: CropBox) {
        self.crop_boxes.push(crop_box);
    }

    pub fn crop_all_boxes(&self) -> Vec<DynamicImage> {
        let mut result = vec![];
        let image_data = self.get_image_data();
        for crop_box in &self.crop_boxes {
            let cropped_image =
                image_data.crop_imm(crop_box.x, crop_box.y, crop_box.width, crop_box.height);
            result.push(cropped_image);
        }
        result
    }

    pub fn get_image_name(&self) -> String {
        let base_name = self.path.file_name();
        let image_name_option = match base_name {
            Some(os_str_name) => os_str_name.to_str(),
            None => Option::Some("None"),
        };
        match image_name_option {
            Some(image_name) => String::from(image_name),
            None => String::from("None"),
        }
    }

    pub fn get_iced_image_handle(&self) -> Handle {
        Handle::from_path(&self.path)
    }
}

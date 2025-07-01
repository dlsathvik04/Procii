use std::path::PathBuf;

pub async fn pick_folder() -> Result<PathBuf, String> {
    let file_hanndle_option = rfd::AsyncFileDialog::new().pick_folder().await;
    match file_hanndle_option {
        Some(handle) => Ok(handle.path().to_path_buf()),
        None => Err(String::from("File dialog closed")),
    }
}

use std::fs::{self};
use std::io;
use std::path::PathBuf;

fn add_file_paths_to_vector(
    path: &PathBuf,
    image_paths: &mut Vec<PathBuf>,
    extensions: &Vec<&str>,
) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;

            let entry_path = entry.path();
            if entry_path.is_dir() {
                add_file_paths_to_vector(&entry_path, image_paths, extensions).unwrap();
            } else if extensions.contains(
                &entry_path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            ) {
                image_paths.push(entry_path);
            }
        }
    }
    Ok(())
}

pub fn get_file_paths_with_extensions(path: PathBuf, extensions: &Vec<&str>) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = vec![];
    add_file_paths_to_vector(&path, &mut file_paths, extensions).unwrap();
    file_paths
}

pub fn get_all_image_paths(path: PathBuf) -> Vec<PathBuf> {
    let mut image_paths: Vec<PathBuf> = vec![];

    let extensions = vec!["jpg", "png", "jpeg"];

    add_file_paths_to_vector(&path, &mut image_paths, &extensions).unwrap();
    image_paths
}

pub fn get_sub_directories(path: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                result.push(entry_path);
            }
        }
    }
    Ok(result)
}

use std::{io, fs};
use std::ops::Div;
use image::{ImageBuffer, RgbImage, imageops};
use std::path::PathBuf;

#[derive(Debug)]
enum FileType {
    IMAGE,
    VIDEO
}

#[derive(Debug)]
struct File {
    path: String,
    file_type: FileType
}

impl File {
    fn new(path_: String, file_type_: FileType) -> Self {
        File {
            path: path_,
            file_type: file_type_
        }
    }
}

fn get_supported_ext_for_file_type<'a>(file_type: &FileType) -> Vec<&'a str> {
    match file_type {
        FileType::IMAGE => vec![".jpg", ".jpeg", ".png"],
        FileType::VIDEO => vec![".mp4", ".mov"]
    }
}

fn get_files_in_dir_recursive(dir_path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir_path).unwrap();
    let mut files: Vec<PathBuf> = vec![];
    for entry in entries {
        let path = entry.ok().unwrap().path();
        if path.is_file() {
            files.push(path)
        } else {
            let dir = path.to_str().unwrap();
            let paths = get_files_in_dir_recursive(dir).unwrap();
            files.extend(paths);
        }
    }
    Ok(files)
}

fn get_media_files(path: &str) -> Vec<File> {
    let files = get_files_in_dir_recursive(path).unwrap();
    let image_extensions = get_supported_ext_for_file_type(&FileType::IMAGE);
    let video_extensions = get_supported_ext_for_file_type(&FileType::VIDEO);
    let mut media_files = vec![];

    for file in files {
        for img_ext in &image_extensions {
            if file.file_name().unwrap().to_str().unwrap().contains(img_ext) {
                media_files.push(File::new(file.to_str().unwrap().to_owned(), FileType::IMAGE))
            }
        }

        for vid_ext in &video_extensions {
            if file.file_name().unwrap().to_str().unwrap().contains(vid_ext) {
                media_files.push(File::new(file.to_str().unwrap().to_owned(), FileType::VIDEO))
            }
        }
    }
    media_files
}

fn main() {
    const TARGET_SIZE: (u32, u32) = (256, 256);
    let mut files = get_media_files("/Users/teepy/Pictures");
    let files2 = get_media_files("/Users/teepy/Downloads");
    files.extend(files2);

    let grid_size = f32::sqrt(files.len() as f32).ceil() as u32;
    let total_width = TARGET_SIZE.0 * grid_size;
    let total_height = TARGET_SIZE.1 * grid_size;

    let mut collage_buffer: RgbImage = ImageBuffer::new(total_width, total_height);

    for (index, file) in files.iter().enumerate() {
        match file.file_type {
            FileType::IMAGE => {
                println!("Start for {}", file.path.clone());
                match image::open(file.path.clone()) {
                    Ok(frame) => {
                        let img = frame.resize_exact(TARGET_SIZE.0, TARGET_SIZE.1, imageops::FilterType::Triangle);
                        let col: u32 = (index as u32) % grid_size;
                        let row = (index as f32).div(grid_size as f32).round() as u32;
                        let x = (col * TARGET_SIZE.0) as i64;
                        let y = (row * TARGET_SIZE.1) as i64;

                        match img.as_rgb8() {
                            Some(img_) => imageops::overlay(&mut collage_buffer, img_, x, y),
                            None => println!("Reading returns None")
                        }
                    },
                    Err(_) => println!("Error reading frame...")
                }
            },
            FileType::VIDEO => ()
        }
    };
    collage_buffer.save("result.png").unwrap();
}



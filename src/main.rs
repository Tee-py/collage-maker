use std::{fs};
use std::ops::Div;
use image::{ImageBuffer, RgbImage, imageops, DynamicImage};
use std::path::PathBuf;
use clap::{Parser};

#[derive(Parser, Default, Debug)]
struct Arguments {
    /// comma(',') separated string of paths to folders where pictures resides on the system
    #[arg(short = 'p', long = "paths", required = true, value_parser, value_delimiter = ',', num_args = 1..)]
    paths: Vec<String>,
    /// Path where the output image is saved on disk
    #[arg(short = 'o', long = "to_file", required = true)]
    output_file: String
}

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

fn get_supported_ext_for_file_type(file_type: &FileType) -> Vec<&str> {
    match file_type {
        FileType::IMAGE => vec![".jpg", ".jpeg", ".png"],
        FileType::VIDEO => vec![".mp4", ".mov"]
    }
}

fn get_files_in_dir_recursive(dir_path: &str) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir_path).unwrap();
    let mut files: Vec<PathBuf> = vec![];
    for entry in entries {
        let path = entry.ok().unwrap().path();
        if path.is_file() {
            files.push(path)
        } else {
            let dir = path.to_str().unwrap();
            let paths = get_files_in_dir_recursive(dir);
            files.extend(paths);
        }
    }
    files
}

fn get_media_files(path: &str) -> Vec<File> {
    let files = get_files_in_dir_recursive(path);
    let mut media_files = vec![];

    for file in files {
        for file_type in [FileType::IMAGE, FileType::VIDEO] {
            for ext in get_supported_ext_for_file_type(&file_type) {
                if file.file_name().unwrap().to_str().unwrap().contains(ext) {
                    media_files.push(File::new(file.to_str().unwrap().to_owned(), FileType::IMAGE))
                }
            }
        }
    }
    media_files
}

fn process_grid_image(path: &String, target_width: u32, target_height: u32) -> Option<DynamicImage> {
    match image::open(path) {
        Ok(img) => Some(img.resize_exact(target_width, target_height, imageops::FilterType::Triangle)),
        Err(e) => {eprintln!("Error processing grid: {:?}", e); None}
    }
}

fn paste_grid(buffer: &mut RgbImage, img: DynamicImage, row: u32, col: u32, target_width: u32, target_height: u32) {
    let x = (col * target_width) as i64;
    let y = (row * target_height) as i64;

    match img.as_rgb8() {
        Some(img_) => imageops::overlay(buffer, img_, x, y),
        None => eprintln!("Invalid grid...")
    }
}

fn main() {
    let args = Arguments::parse();
    let media_paths = args.paths;
    let output_path = args.output_file;

    const TARGET_SIZE: (u32, u32) = (256, 256);
    let mut media_files = vec![];
    for path in media_paths {
        media_files.extend(get_media_files(&path))
    }

    let grid_size = f32::sqrt(media_files.len() as f32).ceil() as u32;
    let total_width = TARGET_SIZE.0 * grid_size;
    let total_height = TARGET_SIZE.1 * grid_size;
    let mut collage_buffer: RgbImage = ImageBuffer::new(total_width, total_height);

    for (index, file) in media_files.iter().enumerate() {
        match file.file_type {
            FileType::IMAGE => {
                let col: u32 = (index as u32) % grid_size;
                let row = (index as f32).div(grid_size as f32).round() as u32;
                match process_grid_image(&file.path.clone(), TARGET_SIZE.0, TARGET_SIZE.1) {
                    Some(grid_img) => paste_grid(&mut collage_buffer, grid_img, row, col, TARGET_SIZE.0, TARGET_SIZE.1),
                    None => ()
                };
            },
            FileType::VIDEO => ()
        }
    };
    collage_buffer.save(output_path).unwrap();
}

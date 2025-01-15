mod config;

use config::*;
use std::{
    error::Error,
    fs::{self, *},
    path::PathBuf,
};

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() <= 1 {
            return Err("not enough arguments");
        }

        let path: String = args[1].clone();

        Ok(Config {
            path: path.to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let root_path: PathBuf = PathBuf::from(config.path);
    let (folders, files, total_size) = analyze_directory(&root_path)?;

    println!("\nStatus ✨");
    println!("folders: {}", folders);
    println!("files: {}", files);
    println!(
        "size: {}",
        match total_size {
            0..=MAX_BYTES => format!("{} bytes", total_size),
            KB..=MAX_KB => format!("{:.2} KB", total_size as f64 / KB as f64),
            MB..=MAX_MB => format!("{:.2} MB", total_size as f64 / MB as f64),
            _ => format!("{:.2} GB", total_size as f64 / GB as f64),
        }
    );

    Ok(())
}

fn analyze_directory(path: &PathBuf) -> Result<(u64, u64, u64), Box<dyn Error>> {
    let mut folders: u64 = 0;
    let mut files: u64 = 0;
    let mut total_size: u64 = 0;

    if path.is_dir() {
        folders += 1;

        let contents: ReadDir = fs::read_dir(path)?;
        for entry in contents {
            let entry: DirEntry = entry?;
            let path: PathBuf = entry.path();

            if path.is_dir() {
                let (sub_folders, sub_files, sub_size) = analyze_directory(&path)?;
                folders += sub_folders;
                files += sub_files;
                total_size += sub_size;
            } else if path.is_file() {
                files += 1;
                total_size += fs::metadata(&path)?.len();
            }
        }
    }

    Ok((folders, files, total_size))
}

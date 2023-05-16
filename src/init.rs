use std::{
    env,
    fs::{self},
    path::PathBuf,
};
use time::OffsetDateTime;
use walkdir::{DirEntry, WalkDir};
use indicatif::{ProgressBar, ProgressStyle};

use crate::hashing::{self, save_csv, CsvData};

pub fn init_record(path: Option<PathBuf>) {
    let path: PathBuf = match path {
        Some(p) => p,
        None => env::current_dir().unwrap(),
    };

    check_init_folder_exist(&path);
}

fn check_init_folder_exist(path: &PathBuf) {
    let mut init_path: PathBuf = path.clone();
    init_path.push(".file-diff");
    if init_path.exists() {
        println!("record folder existed");
    } else {
        fs::create_dir(&init_path).unwrap();
        println!("folder created");

        let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

        let mut data: Vec<CsvData> = Vec::new();
        let walker = WalkDir::new(&path).into_iter();
        let total_count = walker.count();
        let pb = ProgressBar::new(total_count.try_into().unwrap());
        let walker = WalkDir::new(&path).into_iter();
        pb.set_style(spinner_style.clone());
        let mut count = 0;
        for file in walker
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok())
        {
            if file.metadata().unwrap().is_file() {
                let file_path: String = String::from(file.path().to_string_lossy());
                // println!("get hash from {} ", file_path);
                pb.set_style(spinner_style.clone());
                // pb.set_prefix(format!("get hash from {} ", file_path));
                pb.set_message(format!("get hash from {} ", file_path));
                pb.inc(1);
                let csv_data: CsvData = hashing::get_file_hash(file_path);

                data.push(csv_data);
                count+=1;
            }
            
        }
        
        let local = OffsetDateTime::now_local().unwrap();

        init_path.push(format!("{}_init", local.unix_timestamp()));
        save_csv(init_path, data);
        pb.finish_with_message(format!("Done! Total File:{count}"));
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

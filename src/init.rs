use std::{
    env,
    fs::{self},
    path::PathBuf,
};
use walkdir::{DirEntry, WalkDir};

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
        println!("folder existed");
    } else {
        fs::create_dir(&init_path).unwrap();
        println!("folder created");

        let mut data: Vec<CsvData> = Vec::new();
        let walker = WalkDir::new(&path).into_iter();
        for file in walker
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok())
        {
            if file.metadata().unwrap().is_file() {
                let file_path: String = String::from(file.path().to_string_lossy());
                println!("get hash from {} ", file_path);
                let csv_data: CsvData = hashing::get_file_hash(file_path);

                data.push(csv_data);
            }
        }

        init_path.push("init.csv");
        save_csv(init_path, data);
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

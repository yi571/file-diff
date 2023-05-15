use std::{path::PathBuf, env};

use time::OffsetDateTime;
use walkdir::{DirEntry, WalkDir};

use crate::hashing::{CsvData, self, save_csv};


pub fn record(msg:String, path: Option<PathBuf>){
    let path: PathBuf = match path {
        Some(p) => p,
        None => env::current_dir().unwrap(),
    };

    let mut init_path = path.clone();
    init_path.push(".file-diff");

    if init_path.exists(){
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
        let local = OffsetDateTime::now_local().unwrap();
        init_path.push(format!("{}_{}.csv", local.unix_timestamp(), msg));
        save_csv(init_path, data);

    } else {
        println!("folder not exist");
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
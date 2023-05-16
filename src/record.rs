use std::{path::PathBuf, env};

use time::OffsetDateTime;
use walkdir::{DirEntry, WalkDir};
use indicatif::{ProgressBar, ProgressStyle};

use crate::hashing::{CsvData, self, save_csv};


pub fn record(msg:String, path: Option<PathBuf>){
    let path: PathBuf = match path {
        Some(p) => p,
        None => env::current_dir().unwrap(),
    };

    let mut init_path = path.clone();
    init_path.push(".file-diff");

    if init_path.exists(){
        let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
        let mut data: Vec<CsvData> = Vec::new();
        let walker = WalkDir::new(&path).into_iter();
        let total_count = walker.count();
        let pb = ProgressBar::new(total_count.try_into().unwrap());
        pb.set_style(spinner_style.clone());
        let walker = WalkDir::new(&path).into_iter();
        let mut count = 0;
        for file in walker
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok())
        {
            if file.metadata().unwrap().is_file() {
                let file_path: String = String::from(file.path().to_string_lossy());
                // println!("get hash from {} ", file_path);
                pb.set_style(spinner_style.clone());
                pb.set_message(format!("get hash from {} ", file_path));
                pb.inc(1);
                let csv_data: CsvData = hashing::get_file_hash(file_path);
                
                data.push(csv_data);
                count+=1;
            }
        }
        
        let local = OffsetDateTime::now_local().unwrap();
        init_path.push(format!("{}_{}", local.unix_timestamp(), msg));
        save_csv(init_path, data);
        pb.finish_with_message(format!("Done! Total File:{count}"));
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
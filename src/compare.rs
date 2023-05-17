use std::{path::PathBuf, env};
use time::OffsetDateTime;

use crate::hashing::CompareCsvData;

pub fn compare(
    path: Option<PathBuf>,
    origin_item: String,
    target_item: String,
    export_path: Option<PathBuf>,
) {
    let path: PathBuf = match path {
        Some(p) => p,
        None => env::current_dir().unwrap(),
    };

    let mut init_path = path.clone();
    init_path.push(".file-diff");

    if init_path.exists() {
        let mut originit_path = init_path.clone();
        originit_path.push(&origin_item);

        let mut target_path = init_path.clone();
        target_path.push(&target_item);

        if !originit_path.exists() {
            println!("{} not exist", origin_item);
        }

        if !target_path.exists() {
            println!("{} not exist", target_item);
        }

        let result:Vec<CompareCsvData> = CompareCsvData::new(&originit_path.to_string_lossy(), &target_path.to_string_lossy());
        println!("| path | {origin_item} | {target_item}|");
        println!("| ----- | ----- | ----- |");
        for item in &result {
            println!("| {} | {} | {} |", item.path, item.origin_hash, item.target_hash);
        }

        match export_path{
            Some(p) => {
                let mut csv_path = p.clone();

                let local = OffsetDateTime::now_local().unwrap();
                csv_path.push(format!("{}_compare.csv", local.unix_timestamp()));


                CompareCsvData::save_to_csv(csv_path, result, origin_item, target_item)
            },
            None => {},
        }
    } else {
        println!("record folder not exist");
    }
}

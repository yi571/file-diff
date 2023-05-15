use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs::File, io, path::PathBuf};

#[derive(Serialize, Debug, Deserialize)]
pub struct CsvData {
    pub path: String,
    pub hash: String,
}

pub fn get_file_hash(file_path: String) -> CsvData {
    let mut file = File::open(&file_path).unwrap();
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).unwrap();
    let result = hasher.finalize();
    let hash_data = format!("{:x}", result);
    
    CsvData { path: file_path, hash: hash_data }
}

pub fn save_csv(csv_path: PathBuf, data: Vec<CsvData>) {
    
    let mut wtr = csv::Writer::from_path(csv_path).expect("csv init error");
    wtr.write_record(&["path", "hash"]).expect("write head error");

    for i in data{
        wtr.write_record(&[i.path, i.hash]).expect("write data error");
    }

    wtr.flush().expect("save csv error")
}


impl CsvData {
    pub fn get_data_from_csv(csv_path: &str) -> Vec<CsvData> {
        // Open file
        let file = std::fs::File::open(csv_path).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut data:Vec<CsvData> = Vec::new();

        // push all the records
        for result in rdr.records().into_iter() {
            let record = result.unwrap();
            
            let csv_data = Self::get_stuct(&record);
            data.push(csv_data);
         }

         return data;
    }

    fn get_stuct(row: &csv::StringRecord) -> CsvData  {
        let csv_data = CsvData{
            path: row[0].to_string(),
            hash: row[1].to_string(),
        };
        csv_data
    }
}

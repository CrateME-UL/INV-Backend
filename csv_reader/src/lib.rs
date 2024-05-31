use std::error::Error;
use std::fs::File;
use std::path::Path;

use csv::ReaderBuilder;
use serde::de::DeserializeOwned;

pub fn read_csv<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let file = File::open(Path::new(file_path))?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

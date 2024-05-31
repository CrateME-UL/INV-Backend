use std::error::Error;
use std::path::Path;

use calamine::{open_workbook, Reader, Xlsx};
use serde::de::DeserializeOwned;
use serde_json::from_value;

pub fn read_excel<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    // Open the Excel file
    let mut workbook: Xlsx<_> = open_workbook(Path::new(file_path))?;

    // Get the first sheet
    let range = workbook
        .worksheet_range_at(0)
        .ok_or("Cannot find the first sheet")??;

    // Collect the records
    let mut records = Vec::new();
    let headers: Vec<String> = range
        .rows()
        .next()
        .ok_or("Empty sheet")?
        .iter()
        .map(|cell| cell.to_string())
        .collect();

    for row in range.rows().skip(1) {
        let record_map: serde_json::Map<String, serde_json::Value> = headers
            .iter()
            .zip(row.iter())
            .map(|(header, cell)| (header.clone(), serde_json::Value::String(cell.to_string())))
            .collect();

        let record: T = from_value(serde_json::Value::Object(record_map))?;
        records.push(record);
    }

    Ok(records)
}

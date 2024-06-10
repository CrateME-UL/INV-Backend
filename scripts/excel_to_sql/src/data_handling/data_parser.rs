use crate::read_excel;

use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub place: String,
    pub obj: String,
    pub qte: String,
    pub emp: String,
}

#[derive(Debug, Clone)]
pub struct ParsedData {
    pub place: String,
    pub obj: String,
    pub qte: i32,
    pub emp: String,
}
pub fn parse_records_from_excel(file_path: &str) -> Result<Vec<ParsedData>, Box<dyn Error>> {
    let records = read_excel::<Data>(&file_path)?;
    let mut parsed_data: Vec<ParsedData> = Vec::new();
    let mut i = 0;
    for record in records {
        match record.to_parsed_data() {
            Some(data) => parsed_data.push(data),
            None => i += 1,
        }
    }
    println!("nb of parsing errors: {} ", i);

    Ok(parsed_data)
}

impl Data {
    pub fn to_parsed_data(&self) -> Option<ParsedData> {
        let parse_error_status = -1;
        let max_name_len = 30;
        let max_emp_len = 30;

        let int_parse_value = match self.qte.parse::<i32>() {
            Ok(value) => value,
            Err(_) => parse_error_status,
        };

        if int_parse_value == parse_error_status
            || self.obj.len() > max_name_len
            || self.place.len() > max_name_len
            || self.emp.len() > max_emp_len
        {
            return None;
        }
        Some(ParsedData {
            place: self.place.clone(),
            obj: self.obj.clone(),
            qte: int_parse_value,
            emp: self.emp.clone(),
        })
    }
}

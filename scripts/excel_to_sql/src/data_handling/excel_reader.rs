use std::error::Error;
use std::path::Path;

use calamine::{open_workbook, Reader, Xlsx};
use serde::de::DeserializeOwned;
use serde_json::from_value;

pub fn read_excel<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let mut workbook: Xlsx<_> = open_workbook(Path::new(file_path))?;

    let range = workbook
        .worksheet_range_at(0)
        .ok_or("Cannot find the first sheet")??;

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

#[cfg(test)]
mod read_excel_tests {
    use super::*;
    use serde::Deserialize;
    use tempfile::TempDir;
    use xlsxwriter::Workbook;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestData {
        header1: String,
        header2: String,
    }

    fn create_excel_fixture() -> (TempDir, std::path::PathBuf) {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.xlsx");

        let workbook =
            Workbook::new(file_path.to_str().unwrap()).expect("Failed to create workbook");
        let mut sheet = workbook
            .add_worksheet(None)
            .expect("Failed to add worksheet");
        sheet.write_string(0, 0, "header1", None).unwrap();
        sheet.write_string(0, 1, "header2", None).unwrap();
        sheet.write_string(1, 0, "value1", None).unwrap();
        sheet.write_string(1, 1, "value2", None).unwrap();
        sheet.write_string(2, 0, "value3", None).unwrap();
        sheet.write_string(2, 1, "value4", None).unwrap();
        workbook.close().expect("Failed to close workbook");

        (dir, file_path)
    }

    #[test]
    fn given_wrong_file_path_when_reading_excel_then_result_is_err() {
        let file_path = "non_existent_file.xlsx";

        let result: Result<Vec<TestData>, Box<dyn Error>> = read_excel(file_path);

        assert!(result.is_err());
    }

    #[test]
    fn given_no_sheet_when_reading_excel_then_result_is_err() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.xlsx");
        let workbook =
            Workbook::new(file_path.to_str().unwrap()).expect("Failed to create workbook");
        workbook.close().expect("Failed to close workbook");

        let result: Result<Vec<TestData>, Box<dyn Error>> = read_excel(file_path.to_str().unwrap());

        assert!(result.is_err());
    }

    #[test]
    fn given_no_line_when_reading_excel_then_data_len_is_zero() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.xlsx");
        let workbook =
            Workbook::new(file_path.to_str().unwrap()).expect("Failed to create workbook");
        let mut sheet = workbook
            .add_worksheet(None)
            .expect("Failed to add worksheet");
        sheet.write_string(0, 0, "header1", None).unwrap();
        sheet.write_string(0, 1, "header2", None).unwrap();
        workbook.close().expect("Failed to close workbook");
        let expected_nb_rows = 0;

        let result: Result<Vec<TestData>, Box<dyn Error>> = read_excel(file_path.to_str().unwrap());
        let data = result.unwrap();

        assert_eq!(data.len(), expected_nb_rows);
    }

    #[test]
    fn given_valid_file_path_when_reading_excel_then_result_is_ok() {
        let (_dir, file_path) = create_excel_fixture();

        let result: Result<Vec<TestData>, Box<dyn Error>> = read_excel(file_path.to_str().unwrap());

        assert!(result.is_ok());
    }

    #[test]
    fn given_valid_file_path_when_reading_excel_then_returns_all_data_rows() {
        let (_dir, file_path) = create_excel_fixture();
        let expected_nb_rows = 2;

        let result: Result<Vec<TestData>, Box<dyn Error>> = read_excel(file_path.to_str().unwrap());

        let data = result.unwrap();
        assert_eq!(data.len(), expected_nb_rows);
    }

    #[test]
    fn given_valid_file_path_when_reading_excel_then_row_equals_to_file_row() {
        let (_dir, file_path) = create_excel_fixture();
        let first_data_row_index = 0;
        let result: Result<Vec<TestData>, Box<dyn Error>> = read_excel(file_path.to_str().unwrap());

        let data = result.unwrap();
        assert_eq!(
            data[first_data_row_index],
            TestData {
                header1: "value1".to_string(),
                header2: "value2".to_string()
            }
        );
    }
}

//* the module api to call */
mod data_handling;

//* crates that will be available to use in excel_to_sql (root) scope*/
pub use crate::data_handling::data_parser::{parse_records_from_excel, Data, ParsedData};
pub use crate::data_handling::excel_reader::read_excel;
pub use crate::data_handling::sql_handler::{add_inventory_db, add_items_db, add_places_db};

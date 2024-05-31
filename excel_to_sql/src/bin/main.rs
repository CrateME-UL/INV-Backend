use excel_to_sql::read_excel;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    place: String,
    obj: String,
    qte: String,
    emp: String,
}

fn main() {
    println!("Hello, world!");
    parse_records_from_excel();
}

fn parse_records_from_excel() {
    match read_excel::<Data>("map_inventaire.xlsx") {
        Ok(records) => {
            for record in records {
                //TODO print only the rows that are problematic, and skip them
                //TODO remove the problematic data from the Vec<Data>
                //TODO add to database the places first, then the objects for each place (make sure to map the places first)
                println!("{:?}; {:?}; {:?}; {:?}; ", record.place, record.obj, record.qte, record.emp);
            }
        }
        Err(e) => eprintln!("Error reading excel file: {}", e),
    }
}
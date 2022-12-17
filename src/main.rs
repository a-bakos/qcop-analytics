mod consts;
mod csv;
mod data_processor;
mod records;
mod search_query;

use crate::csv::{parse_csv, write_to_csv};
use crate::records::RecordCollection;

fn main() {
    let mut collection: RecordCollection = RecordCollection::new();
    let _parse = parse_csv(consts::CSV_INPUT_FILE_NAME, &mut collection);
    let _write = write_to_csv(consts::CSV_OUTPUT_FILE_NAME, collection);
    println!("Finished.");
}

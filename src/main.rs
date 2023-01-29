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
    println!("{:#?}", collection);
    collection.show_stats();

    println!("Sorting by counter...");
    collection.sort_by_counter();

    let _write_csv_main = write_to_csv(consts::CSV_OUTPUT_FILE_NAME, &collection, 1);
    let _write_csv_order_by_count = write_to_csv(consts::CSV_OUTPUT_FILE_NAME_ORDER_BY_COUNT, &collection, 2);

    println!("Finished.");
}

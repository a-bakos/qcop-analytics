mod consts;
mod csv;
mod intelligence;
mod data_processor;
mod records;
mod search_query;

use crate::csv::{parse_csv_into_collection, write_to_csv};
use crate::records::RecordCollection;

fn main() {
    // Some configuration parameters
    println!("[Config] CSV to parse: {}", consts::CSV_INPUT_FILE_NAME);
    if consts::EXCLUDE_LOGGED_IN_USER_SEARCHES {
        println!("[Config] Excluding logged in user searches\n");
    }

    let mut collection: RecordCollection = RecordCollection::new();
    let _parse = parse_csv_into_collection(consts::CSV_INPUT_FILE_NAME, &mut collection);

    let _write_csv_main = write_to_csv(
        consts::CSV_OUTPUT_FILE_NAME,
        &collection,
        records::CollectionType::Main,
    );

    println!("\nSorting by counter...");
    collection.sort_by_counter();
    let _write_csv_order_by_count = write_to_csv(
        consts::CSV_OUTPUT_FILE_NAME_ORDER_BY_COUNT,
        &collection,
        records::CollectionType::OrderByCount,
    );
    println!("{:#?}", collection.map_by_counter);

    // todo
    /*println!("\nSorting by target...");
    collection.sort_by_target();
    let _write_csv_order_by_target = write_to_csv(
        consts::CSV_OUTPUT_FILE_NAME_ORDER_BY_TARGET,
        &collection,
        records::CollectionType::OrderByTarget,
    );*/

    println!("Finished.");
}

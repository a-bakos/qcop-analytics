mod consts;
mod csv;
mod data_filters;
mod intelligence;
mod records;

use crate::{
    csv::{parse_csv_into_collection, write_to_csv},
    records::RecordCollection,
};

fn main() {
    println!("\n[ QCop Intel - Search Insights ]\n");

    // Configuration parameter notices
    println!("[Config] CSV to parse: {}", consts::CSV_INPUT_FILE_NAME);
    println!(
        "[Config] Keyword MIN / MAX length: {} / {}",
        consts::KEYWORD_MIN_LENGTH,
        consts::KEYWORD_MAX_LENGTH
    );
    if consts::EXCLUDE_LOGGED_IN_USER_SEARCHES {
        println!("[Config] Excluding logged in user searches");
    }
    if consts::EXCLUDE_ONLY_NUMBER_SEARCHES {
        println!("[Config] Excluding number-only entries");
    }
    println!(
        "[Config] Number of top keywords: {}",
        consts::NUMBER_OF_TOP_KEYWORDS
    );
    println!(
        "[Config] Number of top targets: {}",
        consts::NUMBER_OF_TOP_TARGETS
    );
    println!("\n");

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

    println!("\nSorting by target...");
    collection.sort_by_target();
    let _write_csv_order_by_target = write_to_csv(
        consts::CSV_OUTPUT_FILE_NAME_ORDER_BY_TARGET,
        &collection,
        records::CollectionType::OrderByTarget,
    );

    println!("\nFinding top keywords...");
    collection.find_top_keywords();
    let _write_csv_top_keywords = write_to_csv(
        consts::CSV_OUTPUT_FILE_NAME_TOP_KEYWORDS,
        &collection,
        records::CollectionType::TopKeywords,
    );

    // todo
    // println!("\nFinding top targets...");
    // collection.find_top_targets();

    println!("\n[ QCop finished. ]");
}

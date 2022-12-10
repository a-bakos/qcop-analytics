mod csv;
mod search_query;

use std::collections::HashMap;

use crate::csv::{parse_csv, write_to_csv, RecordCollection};
use search_query::*;

const CSV_INPUT_FILE_NAME: &'static str = "export/er-wp_q_cop.csv"; //"export/as-wp_q_cop.csv";
const CSV_OUTPUT_FILE_NAME: &'static str = "outtest.csv";

fn main() {
    let mut collection: RecordCollection = RecordCollection {
        map: HashMap::new(),
    };

    println!("Parsing CSV...");
    parse_csv(self::CSV_INPUT_FILE_NAME, &mut collection);
    println!("Parsing finished.");
    println!("Writing results into CSV...");
    write_to_csv(self::CSV_OUTPUT_FILE_NAME, collection);
    println!("Finished.");
}

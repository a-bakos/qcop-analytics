mod csv;
mod search_query;

use std::collections::HashMap;

use crate::csv::{parse_csv, write_to_csv, RecordCollection};
use search_query::*;

const CSV_INPUT_FILE_NAME: &'static str = "export/test.csv"; //"export/as-wp_q_cop.csv";
const CSV_OUTPUT_FILE_NAME: &'static str = "outtest.csv";

fn main() {
    let mut collection: RecordCollection = RecordCollection {
        map: HashMap::new(),
    };

    parse_csv(self::CSV_INPUT_FILE_NAME, &mut collection);
    write_to_csv(self::CSV_OUTPUT_FILE_NAME, collection);

    // println!("{:#?}", collection);
}

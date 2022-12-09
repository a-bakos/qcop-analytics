mod csv;
mod search_query;

use std::collections::HashMap;

use crate::csv::{parse_csv, RecordCollection};
use search_query::*;

const CSV_FILE_NAME: &'static str = "export/test.csv"; //"export/as-wp_q_cop.csv";

fn main() {
    let mut collection: RecordCollection = RecordCollection {
        map: HashMap::new(),
    };

    parse_csv(self::CSV_FILE_NAME, &mut collection);

    println!("{:#?}", collection);
}

mod csv;
mod search_query;

use crate::csv::parse_csv;
use search_query::*;

const CSV_FILE_NAME: &'static str = "export/test.csv"; //"export/as-wp_q_cop.csv";

fn main() {
    parse_csv(self::CSV_FILE_NAME);
}

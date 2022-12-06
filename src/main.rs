mod csv;
mod search_query;

use crate::csv::parse_csv;
use search_query::*;

const CSV_FILE_NAME: &'static str = "export/test.csv"; //"export/as-wp_q_cop.csv";

fn main() {
    let query_test: SearchQuery = SearchQuery::new((
        String::from("2018-08-21 17:19:46"),
        String::from("sharing your work"),
        String::from("https%3A%2F%2Fauthorservices.taylorandfrancis.com%2F"),
        32,
        String::from("https://authorservices.taylorandfrancis.com/sharing-your-work"),
    ));
    println!("{:#?}", query_test);

    parse_csv(self::CSV_FILE_NAME);
}

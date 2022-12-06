extern crate csv;

use csv::{Reader, StringRecord};
use std::error::Error;

#[derive(Debug)]
struct CleanRecord {
    date_time: String,
    keyword: String,
    source: String,
    hits: String,
    target: String,
}

impl CleanRecord {
    fn new(
        date_time: String,
        keyword: String,
        source: String,
        hits: String,
        target: String,
    ) -> Self {
        Self {
            date_time,
            keyword,
            source,
            hits,
            target,
        }
    }
}

pub fn parse_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    for row in reader.records() {
        let query_row: StringRecord = row?;

        let date_time: String = match query_row.get(1) {
            Some(datetime) => {
                // run datetime parser here if needed
                datetime.trim().to_string()
            }
            None => String::from(""),
        };
        let keyword: String = match query_row.get(2) {
            Some(keyword) => {
                // run keyword parser here if needed
                keyword.trim().to_string()
            }
            None => String::from(""),
        };
        let source: String = match query_row.get(3) {
            Some(source) => {
                // run source parser here if needed
                // + url decoder
                source.trim().to_string()
            }
            None => String::from(""),
        };
        let hits: String = match query_row.get(4) {
            Some(hits) => hits.trim().to_string(),
            None => String::from(""),
        };
        let target: String = match query_row.get(8) {
            Some(target) => {
                // run target parser here if needed
                // + url decoder
                target.trim().to_string()
            }
            None => String::from(""),
        };

        let clean_record = CleanRecord::new(date_time, keyword, source, hits, target);
        println!("{:#?}", clean_record);
    }
    Ok(())
}

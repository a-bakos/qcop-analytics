extern crate csv;

use crate::consts;
use crate::data_processor;
use crate::records;
use csv::{Reader, StringRecord, WriterBuilder};
use std::error::Error;

pub fn parse_csv(
    file_path: &str,
    collection: &mut records::RecordCollection,
) -> Result<(), Box<dyn Error>> {
    println!("Parsing CSV...");
    let mut reader = Reader::from_path(file_path)?;
    let mut skipped_items: u32 = 0;
    for row in reader.records() {
        let query_row: StringRecord = row?;

        // Check if we need to skip known-user searches
        if let Some(user) = query_row.get(5) {
            if consts::EXCLUDE_LOGGED_IN_USER_SEARCHES && !user.is_empty() {
                skipped_items += 1;
                continue;
            }
        };

        // Check keyword length validity (bounds check)
        let keyword: String = match query_row.get(2) {
            Some(keyword) => {
                // Skip, if length not valid
                if !data_processor::is_valid_length(keyword) {
                    skipped_items += 1;
                    continue;
                }

                let processed_kw = data_processor::keyword(keyword);
                data_processor::handle_if_meaningful(processed_kw.as_str(), collection);

                // Skip, if keyword is invalid
                if processed_kw == consts::DEFAULT_KEYWORD_INVALID {
                    skipped_items += 1;
                    continue;
                }
                processed_kw
            }
            None => data_processor::default(),
        };

        let date_time: String = match query_row.get(1) {
            Some(datetime) => data_processor::datetime(datetime),
            None => data_processor::default(),
        };
        let source: String = match query_row.get(3) {
            Some(source) => data_processor::source_url(source),
            None => data_processor::default(),
        };
        let hits: i32 = match query_row.get(4) {
            Some(hits) => data_processor::hits(hits),
            None => consts::DEFAULT_MISSING_HITS,
        };
        let target: String = match query_row.get(8) {
            Some(target) => data_processor::target_url(target),
            None => data_processor::default(),
        };

        let clean_record = records::CleanRecord::new(date_time, keyword, source, hits, target);
        collection.add(clean_record);
    }
    println!("Parsing finished.");
    println!("Skipped items: {:?}", skipped_items);
    println!("Collection length: {:?}", collection.map.len());
    Ok(())
}

pub fn write_to_csv(
    file_path: &str,
    collection: records::RecordCollection,
) -> Result<(), Box<dyn Error>> {
    println!("Writing results into CSV...");
    let mut wtr = WriterBuilder::new().from_path(file_path)?;

    let collection = collection.map;

    for (key, val) in collection.iter() {
        let keyword = key;
        let count = val.0.to_string();
        wtr.write_record([keyword, count.as_str()])?;
    }

    wtr.flush()?;

    Ok(())
}

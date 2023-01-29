extern crate csv;

use crate::consts;
use crate::data_processor;
use crate::records;
use csv::{Reader, StringRecord, WriterBuilder};
use std::error::Error;

#[allow(non_camel_case_types)]
pub enum CSV_TYPE {
    Main,
    OrderByCount,
}

pub fn parse_csv(
    file_path: &str,
    collection: &mut records::RecordCollection,
) -> Result<(), Box<dyn Error>> {
    println!("CSV to parse: {file_path}");
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
                    collection.add_to_stats(records::STAT_TYPE::InvalidSearch);
                    skipped_items += 1;
                    continue;
                }

                let processed_kw = data_processor::keyword(keyword);
                data_processor::handle_if_meaningful(processed_kw.as_str(), collection);

                // Skip, if keyword is invalid
                if processed_kw == consts::DEFAULT_KEYWORD_INVALID {
                    collection.add_to_stats(records::STAT_TYPE::InvalidSearch);
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
    println!("Skipped items: {skipped_items:?}");
    println!("Collection length: {:?}", collection.map.len());
    Ok(())
}

pub fn write_to_csv(
    file_path: &str,
    collection: &records::RecordCollection,
    csv_type: CSV_TYPE,
) -> Result<(), Box<dyn Error>> {
    println!("Writing results into CSV...");
    let mut wtr = WriterBuilder::new().from_path(file_path)?;

    // TODO
    // All of this can probably be refactored to be more idiomatic.
    // As a first step, I want to get it to produce the outcome I need

    match csv_type {
        CSV_TYPE::Main => {
            let collection = &collection.map;
            for (key, val) in collection.iter() {
                let keyword = key;
                let count = val.counter.to_string();
                wtr.write_record([keyword, count.as_str()])?;
            }
        }
        CSV_TYPE::OrderByCount => {
            let collection = &collection.map_by_counter;
            for (key, val) in collection.iter() {
                // get every keyword and get the corresponding counter and print count -> kw
                let count = key;
                for btreemapentry in val.iter() {
                    let mut kw: &String = &"".to_string(); // default
                    for vecentry in btreemapentry.1.iter() {
                        if kw == &vecentry.keyword {
                            continue;
                        }
                        kw = &vecentry.keyword;
                        wtr.write_record([count.to_string(), kw.to_string()])?;
                    }
                }
            }
        }
    }

    wtr.flush()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fn() {
        assert_eq!("", "")
    }
}

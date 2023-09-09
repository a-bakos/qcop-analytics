extern crate csv;

use csv::{Reader, StringRecord, WriterBuilder};
use std::error::Error;
use crate::{
    consts,
    data_processor,
    records,
    records::CollectionType,
};

pub fn parse_csv_into_collection(
    file_path: &str,
    collection: &mut records::RecordCollection,
) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut skipped_items: u32 = 0;

    println!("Parsing CSV into collection...");

    for row in reader.records() {
        let query_row: StringRecord = row?;

        // Check if we need to skip logged-in user searches
        if let Some(user) = query_row.get(consts::CSV_COLUMN_INDEX_USER) {
            if consts::EXCLUDE_LOGGED_IN_USER_SEARCHES && !user.is_empty() {
                skipped_items += 1;
                continue;
            }
        };

        // Check keyword length validity (bounds check)
        let keyword: String = match query_row.get(consts::CSV_COLUMN_INDEX_QUERY) {
            Some(keyword) => {
                // Skip, if length not valid
                if !data_processor::is_valid_length(keyword) {
                    collection.add_to_stats(records::StatType::InvalidSearch);
                    skipped_items += 1;
                    continue;
                }

                let processed_kw = data_processor::clean_keyword(keyword);
                data_processor::handle_if_meaningful(processed_kw.as_str(), collection);

                // Skip, if keyword is invalid
                if processed_kw == consts::DEFAULT_KEYWORD_INVALID {
                    collection.add_to_stats(records::StatType::InvalidSearch);
                    skipped_items += 1;
                    continue;
                }
                processed_kw
            }
            None => data_processor::default(),
        };

        let date_time: String = match query_row.get(consts::CSV_COLUMN_INDEX_DATETIME) {
            Some(datetime) => data_processor::datetime(datetime),
            None => data_processor::default(),
        };
        let source: String = match query_row.get(consts::CSV_COLUMN_INDEX_URL) {
            Some(source) => data_processor::source_url(source),
            None => data_processor::default(),
        };
        let hits: i32 = match query_row.get(consts::CSV_COLUMN_INDEX_HITS) {
            Some(hits) => data_processor::hits(hits),
            None => consts::DEFAULT_MISSING_HITS,
        };
        let target: String = match query_row.get(consts::CSV_COLUMN_INDEX_TARGET) {
            Some(target) => data_processor::target_url(target),
            None => data_processor::default(),
        };

        let clean_record = records::CleanRecord::new(date_time, keyword, source, hits, target);
        collection.add(clean_record);
    }

    println!("Parsing finished.");
    println!("Skipped items: {skipped_items:?}");
    println!("Collection length: {:?}\n", collection.map.len());
    Ok(())
}

pub fn write_to_csv(
    file_path: &str,
    collection: &records::RecordCollection,
    collection_type: CollectionType,
) -> Result<(), Box<dyn Error>> {
    println!("Writing results into CSV: {}", file_path);
    let mut wtr = WriterBuilder::new().from_path(file_path)?;

    match collection_type {
        CollectionType::Main => {
            let collection = &collection.map;
            for (key, val) in collection.iter() {
                let keyword = key;
                let count = val.counter.to_string();
                wtr.write_record([keyword, count.as_str()])?;
            }
        }
        CollectionType::OrderByCount => {
            let collection = &collection.map_by_counter;
            for (count, keyword_collection) in collection.iter() {
                // get every keyword and get the corresponding counter and print count -> kw
                for kw_entry in keyword_collection.iter() {
                    // Store current keyword and skip on duplicates if found
                    let mut current_keyword: &String = &"".to_string(); // temp keyword init
                    for record in kw_entry.1.iter() {
                        if current_keyword == &record.keyword {
                            continue;
                        }
                        current_keyword = &record.keyword;
                        wtr.write_record([count.to_string(), current_keyword.to_string()])?;
                    }
                }
            }
        }
        CollectionType::OrderByTarget => {
            let collection = &collection.map_by_target;
            // todo
            for (target, count) in collection.iter() {}
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

extern crate csv;

use crate::{consts, data_filters, records, records::CollectionType};
use csv::{Reader, StringRecord, WriterBuilder};
use std::error::Error;

pub fn parse_csv_into_collection(
    file_path: &str,
    collection: &mut records::RecordCollection,
) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut number_of_skipped_items: u32 = 0;

    println!("Parsing CSV into collection...");

    for row in reader.records() {
        let query_row: StringRecord = row?;

        // Check if we need to skip logged-in user searches
        if let Some(user) = query_row.get(consts::CSV_COLUMN_INDEX_USER) {
            if consts::EXCLUDE_LOGGED_IN_USER_SEARCHES && !user.is_empty() {
                number_of_skipped_items += 1;
                continue;
            }
        };

        // Check keyword length validity (bounds check)
        let keyword: String = match query_row.get(consts::CSV_COLUMN_INDEX_QUERY) {
            Some(keyword) => {
                // Skip, if length not valid
                if !data_filters::is_valid_length(keyword) {
                    collection.add_to_stats(records::StatType::InvalidSearch);
                    number_of_skipped_items += 1;
                    continue;
                }

                let processed_kw = data_filters::clean_keyword(keyword);
                data_filters::handle_if_meaningful(processed_kw.as_str(), collection);

                // Skip, if keyword is invalid
                if processed_kw == consts::DEFAULT_KEYWORD_INVALID {
                    collection.add_to_stats(records::StatType::InvalidSearch);
                    number_of_skipped_items += 1;
                    continue;
                }
                processed_kw
            }
            None => data_filters::default(),
        };

        let date_time: String = match query_row.get(consts::CSV_COLUMN_INDEX_DATETIME) {
            Some(datetime) => data_filters::datetime(datetime),
            None => data_filters::default(),
        };
        let source: String = match query_row.get(consts::CSV_COLUMN_INDEX_URL) {
            Some(source) => data_filters::source_url(source),
            None => data_filters::default(),
        };
        let hits: i32 = match query_row.get(consts::CSV_COLUMN_INDEX_HITS) {
            Some(hits) => data_filters::hits(hits),
            None => consts::DEFAULT_MISSING_HITS,
        };
        let target: String = match query_row.get(consts::CSV_COLUMN_INDEX_TARGET) {
            Some(target) => data_filters::target_url(target),
            None => data_filters::default(),
        };

        let clean_record = records::CleanRecord::new(date_time, keyword, source, hits, target);
        collection.add(clean_record);
    }

    println!("Parsing finished.");
    println!("Skipped items: {number_of_skipped_items:?}");
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
            for (target, keyword_meta) in collection.iter() {
                wtr.write_record([target.clone(), keyword_meta.len().to_string()])?;
            }
        }
        CollectionType::TopKeywords => {
            let collection = &collection.top_keywords;
            let mut iteration = 0;
            'outer: for (count, keyword_meta) in collection.iter().rev() {
                for (keyword, _keyword_meta) in keyword_meta.iter() {
                    if iteration >= consts::NUMBER_OF_TOP_KEYWORDS {
                        break 'outer;
                    }
                    wtr.write_record([keyword.clone(), count.clone().to_string()])?;
                    iteration += 1;
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

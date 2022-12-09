extern crate csv;

use csv::{Reader, StringRecord};
use std::{collections::HashMap, error::Error};
//use urlencoding::decode;

#[derive(Debug)]
pub struct CleanRecordContainer {
    list: Vec<CleanRecord>,
}
impl CleanRecordContainer {
    fn add_to_list(&mut self, record: CleanRecord) {
        self.list.push(record);
    }
}

#[derive(Debug)]
pub struct RecordCollection {
    pub map: HashMap<String, (u32, CleanRecordContainer)>,
}

impl RecordCollection {
    pub fn add(&mut self, record: CleanRecord) {
        let keyword: String = record.keyword.clone();
        if self.map.get(&keyword).is_none() {
            let clean_record_container = CleanRecordContainer { list: vec![record] };
            self.map.insert(keyword, (1, clean_record_container));
        } else {
            let values = self.map.get_mut(&keyword).unwrap();
            values.1.add_to_list(record);
            values.0 = values.0 + 1;
        }
    }
}

#[derive(Debug)]
pub struct CleanRecord {
    pub date_time: String,
    pub keyword: String,
    pub source: String,
    pub hits: String,
    pub target: String,
}

impl CleanRecord {
    pub fn new(
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

pub fn parse_csv(file_path: &str, collection: &mut RecordCollection) -> Result<(), Box<dyn Error>> {
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
                // url decoder
                urlencoding::decode(source.trim()).unwrap().into_owned()
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
        //println!("{:#?}", clean_record);
        collection.add(clean_record);
    }
    Ok(())
}

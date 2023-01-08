use crate::consts;
use std::collections::HashMap;

// These variants used to specify the search's type for statistics
pub enum STAT_TYPE {
    DOI,
    InvalidSearch,
}

#[derive(Debug)]
pub struct RecordCollection {
    /// map meaning: [keyword, (count, metadata)]
    pub map: HashMap<String, (u32, CleanRecordContainer)>,
    stats: HashMap<String, u32>,
}

impl RecordCollection {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            stats: HashMap::new(),
        }
    }

    pub fn add(&mut self, record: CleanRecord) {
        let keyword: String = record.keyword.clone();
        if self.map.get(&keyword).is_none() {
            let clean_record_container = CleanRecordContainer { list: vec![record] };
            self.map.insert(keyword, (1, clean_record_container));
        } else {
            let values = self.map.get_mut(&keyword).unwrap();
            values.1.add_to_list(record);
            values.0 += 1;
        }
    }

    pub fn add_to_stats(&mut self, stat: STAT_TYPE) {
        match stat {
            STAT_TYPE::DOI => {
                let mut counter: u32;
                if self.stats.get(consts::STAT_DOI).is_some() {
                    counter = self.stats.get(consts::STAT_DOI).unwrap().clone();
                    counter += 1;
                } else {
                    counter = 1;
                }
                self.stats.insert(consts::STAT_DOI.to_owned(), counter);
            }
            STAT_TYPE::InvalidSearch => {
                let mut counter: u32;
                if self.stats.get(consts::STAT_INVALID).is_some() {
                    counter = self.stats.get(consts::STAT_INVALID).unwrap().clone();
                    counter += 1;
                } else {
                    counter = 1;
                }
                self.stats.insert(consts::STAT_INVALID.to_owned(), counter);
            },
        }
    }

    // TODO better formatting
    pub fn show_stats(&self) {
        println!("{:#?}", self.stats);
    }
}

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
pub struct CleanRecord {
    pub date_time: String,
    pub keyword: String,
    pub source: String,
    pub hits: i32,
    pub target: String,
}

impl CleanRecord {
    pub fn new(
        date_time: String,
        keyword: String,
        source: String,
        hits: i32,
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

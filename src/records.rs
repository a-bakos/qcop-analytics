use crate::consts;
use std::collections::{BTreeMap, HashMap};

// These variants used to specify the search's type for statistics
pub enum STAT_TYPE {
    DOI,
    InvalidSearch,
}

#[derive(Debug)]
pub struct RecordCollection {
    /// map meaning: [keyword, (count, metadata)]
    pub map: BTreeMap<String, CleanRecordContainer>,
    stats: HashMap<String, u32>,
}

impl RecordCollection {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            stats: HashMap::new(),
        }
    }

    pub fn add(&mut self, record: CleanRecord) {
        let keyword: String = record.keyword.clone();
        if self.map.get(&keyword).is_none() {
            let clean_record_container = CleanRecordContainer {
                counter: 1,
                list: vec![record],
            };
            self.map.insert(keyword, clean_record_container);
        } else {
            let values = self.map.get_mut(&keyword).unwrap();
            values.counter += 1;
            values.add_to_list(record);
        }
    }

    pub fn add_to_stats(&mut self, stat: STAT_TYPE) {
        match stat {
            STAT_TYPE::DOI => {
                self.stats
                    .entry(consts::STAT_DOI.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            STAT_TYPE::InvalidSearch => {
                self.stats
                    .entry(consts::STAT_INVALID.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    // TODO better formatting
    pub fn show_stats(&self) {
        println!("{:#?}", self.stats);
    }
}

#[derive(Debug)]
pub struct CleanRecordContainer {
    pub counter: u32,
    list: Vec<CleanRecord>,
}

impl CleanRecordContainer {
    fn add_to_list(&mut self, record: CleanRecord) {
        self.list.push(record);
    }
}

impl PartialEq for CleanRecordContainer {
    fn eq(&self, other: &Self) -> bool {
        // Return true if the CleanRecordContainer (counter) values are equal, false otherwise
        self.counter == other.counter
    }
}

impl Eq for CleanRecordContainer {}

impl PartialOrd for CleanRecordContainer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Return the ordering of the two CleanRecordContainer values
        Some(self.cmp(other))
    }
}

impl Ord for CleanRecordContainer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.counter.cmp(&other.counter)
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

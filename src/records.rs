use crate::consts;
use std::cmp::Ordering;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashMap};

// These variants used to specify the search's type for statistics
pub enum STAT_TYPE {
    DOI,
    InvalidSearch,
}

#[derive(Debug)]
pub struct RecordCollection {
    /// map meaning: [keyword, (count, metadata)]
    pub map: BTreeMap<String, CleanRecordContainer>, // BTrees are inherently ordered by their keys
    stats: HashMap<String, u32>,

    pub map_by_counter: BTreeMap<u32, CleanRecordContainer>,
}

impl RecordCollection {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            stats: HashMap::new(),

            map_by_counter: BTreeMap::new(),
        }
    }

    // TODO
    pub fn sort_by_counter(&mut self) {
        for entry in self.map.iter_mut() {
            println!("{:#?}", entry);

            let the_keyword = entry.0;
            let kw_meta = entry.1.clone();
            let counter = entry.1.counter;

            // if counter is not in self.map_by_counter
            // then add it in as key
            // and add keyword as value IN a new vec of CleanRecordContainer-like list holding CleanRecord's
            // think: references to CleanRecords would work? referencing the items in self.map.list
            // cannot use CleanRecordContainer because we need a simpler struct (ie no counter field in here)

            todo!();
        }
    }

    pub fn add(&mut self, record: CleanRecord) {
        let keyword: String = record.keyword.clone();

        // A-Z
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

#[derive(Debug, Clone)]
pub struct CleanRecordContainer {
    pub counter: u32,
    list: Vec<CleanRecord>,
}

impl CleanRecordContainer {
    fn add_to_list(&mut self, record: CleanRecord) {
        self.list.push(record);
    }
}

#[derive(Debug, Clone)]
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

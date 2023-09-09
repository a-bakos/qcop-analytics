use crate::consts;
use std::collections::{BTreeMap, HashMap};

// The search's type for statistics
pub enum StatType {
    DOI,
    InvalidSearch,
}

// The final collection's type
pub enum CollectionType {
    Main,
    OrderByCount,
    OrderByTarget,
}

// A cleaned/processed search entry with metadata
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

/// A collection of cleaned/processed search entries with counter
/// to store homogenous CleanRecord entries
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

// Final collection of clean records organized into sorted maps
#[derive(Debug)]
pub struct RecordCollection {
    /// map meaning: [keyword, (count, metadata)]
    // BTrees are inherently ordered by their keys
    pub map: BTreeMap<String, CleanRecordContainer>,

    // <count, <target, keyword_meta>>
    pub map_by_counter: BTreeMap<u32, BTreeMap<String, Vec<CleanRecord>>>,
    pub map_by_target: BTreeMap<String, Vec<CleanRecord>>, // => <count<target, kw_meta>>
    // pub map_by_source: BTreeMap<u32, BTreeMap<String, Vec<CleanRecord>>, // => <count<source, kw_meta>>

    stats: HashMap<String, u32>,
}

impl RecordCollection {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            map_by_counter: BTreeMap::new(),
            map_by_target: BTreeMap::new(),
            stats: HashMap::new(),
        }
    }

    /// Add item to main collection map
    pub fn add(&mut self, record: CleanRecord) {
        let keyword: String = record.keyword.clone();

        if self.map.get(&keyword).is_none() {
            self.map.insert(keyword, CleanRecordContainer {
                counter: 1,
                list: vec![record],
            });
        } else {
            let values = self.map.get_mut(&keyword).unwrap();
            values.counter += 1;
            values.add_to_list(record);
        }
    }

    pub fn add_to_stats(&mut self, stat: StatType) {
        match stat {
            StatType::DOI => {
                self.stats
                    .entry(consts::STAT_DOI.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            StatType::InvalidSearch => {
                self.stats
                    .entry(consts::STAT_INVALID.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    pub fn sort_by_counter(&mut self) {
        for entry in self.map.iter_mut() {
            let the_keyword: &String = entry.0;
            let keyword_meta: CleanRecordContainer = entry.1.clone();
            let counter: u32 = entry.1.counter.clone();
            let keyword_meta_list: Vec<CleanRecord> = keyword_meta.list;

            if self.map_by_counter.get(&counter).is_none() {
                let mut keyword_and_meta_holder: BTreeMap<String, Vec<CleanRecord>> = BTreeMap::new();
                keyword_and_meta_holder.insert(the_keyword.clone(), keyword_meta_list);
                self.map_by_counter.insert(counter, keyword_and_meta_holder);
            } else {
                // The "key" (aka the counter) exists, expand the vec
                let entry = self.map_by_counter.get_mut(&counter).unwrap();

                for keyword_meta_entry in keyword_meta_list.iter() {
                    let new_keyword_meta_entry = CleanRecord {
                        date_time: keyword_meta_entry.date_time.clone(),
                        keyword: keyword_meta_entry.keyword.clone(),
                        source: keyword_meta_entry.source.clone(),
                        hits: keyword_meta_entry.hits.clone(),
                        target: keyword_meta_entry.target.clone(),
                    };
                    entry.insert(the_keyword.clone(), vec![new_keyword_meta_entry]);
                }
            }
        }
    }

    pub fn sort_by_target(&mut self) {
        //  BTreeMap<String(target_url), Vec<CleanRecord>>
        let mut target_holder: BTreeMap<String, Vec<CleanRecord>> = BTreeMap::new();

        // Loop through main map
        // BTreeMap<String, CleanRecordContainer<u32, Vec<CleanRecord>>>
        for (_keyword_string, keyword_meta_collection) in self.map.iter_mut() {
            let search_keywords: Vec<CleanRecord> = keyword_meta_collection.list.clone();

            for record in search_keywords.iter() {
                let target = record.target.clone();

                // We need to break up the target string at whitespaces because it's a list of target URLs
                let split_targets = target.split_whitespace();
                for target in split_targets {
                    // If target doesn't exist as the "key", create it, otherwise expand collection
                    if target_holder.get(target).is_none() {
                        target_holder.insert(target.to_string(), vec![CleanRecord {
                            date_time: record.date_time.clone(),
                            keyword: record.keyword.clone(),
                            source: record.source.clone(),
                            hits: record.hits.clone(),
                            target: record.target.clone(),
                        }]);
                    } else {
                        // The "key" (aka the target url) exists, expand the vec
                        let search_keywords_for_target = target_holder.get_mut(target).unwrap();
                        search_keywords_for_target.push(CleanRecord {
                            date_time: record.date_time.clone(),
                            keyword: record.keyword.clone(),
                            source: record.source.clone(),
                            hits: record.hits.clone(),
                            target: record.target.clone(),
                        });
                    }
                }
            }
        }

        self.map_by_target = target_holder;
    }

    // TODO better formatting
    pub fn show_stats(&self) {
        println!("{:#?}", self.stats);
    }
}
use crate::consts;
use std::collections::{BTreeMap, HashMap};

// These variants used to specify the search's type for statistics
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum STAT_TYPE {
    DOI,
    InvalidSearch,
}

#[derive(Debug)]
pub struct RecordCollection {
    /// map meaning: [keyword, (count, metadata)]
    pub map: BTreeMap<String, CleanRecordContainer>, // BTrees are inherently ordered by their keys
    stats: HashMap<String, u32>,

    pub map_by_counter: BTreeMap<u32, BTreeMap<String, Vec<CleanRecord>>>,
    pub map_by_target: BTreeMap<u32, BTreeMap<String, Vec<CleanRecord>>>, // count<target, kw_meta>
                                                                          // pub map_by_source: BTreeMap<u32, BTreeMap<String, Vec<CleanRecord>>, // count<source, kw_meta>
}

impl RecordCollection {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            stats: HashMap::new(),
            map_by_counter: BTreeMap::new(),
            map_by_target: BTreeMap::new(),
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

    pub fn sort_by_counter(&mut self) {
        for entry in self.map.iter_mut() {
            let the_keyword: &String = entry.0;
            let kw_meta: CleanRecordContainer = entry.1.clone();
            let counter: u32 = entry.1.counter;
            let cleanrecord_container: Vec<CleanRecord> = kw_meta.list;

            // step 1
            // if counter is not in self.map_by_counter
            // then add it in as key
            // and add keyword as value IN a new vec of CleanRecordContainer-like list holding CleanRecord's
            // think: references to CleanRecords would work? referencing the items in self.map.list
            // cannot use CleanRecordContainer because we need a simpler struct (ie no counter field in here)

            // step 2
            // think about further grouping options
            // the vec may need to change to a hashmap or btreemap, so:
            // counter: [ "keyword" => Vec<CleanRecord> ]

            if self.map_by_counter.get(&counter).is_none() {
                let mut btreeinner: BTreeMap<String, Vec<CleanRecord>> = BTreeMap::new();
                btreeinner.insert(the_keyword.clone(), cleanrecord_container);
                self.map_by_counter.insert(counter, btreeinner);
            } else {
                // key (aka counter) exists, expand the vec
                let entry = self.map_by_counter.get_mut(&counter).unwrap();
                for cleanrecord in cleanrecord_container.iter() {
                    let newrecord: CleanRecord = CleanRecord {
                        date_time: cleanrecord.date_time.clone(),
                        keyword: cleanrecord.keyword.clone(),
                        source: cleanrecord.source.clone(),
                        hits: cleanrecord.hits,
                        target: cleanrecord.target.clone(),
                    };
                    entry.insert(the_keyword.clone(), vec![newrecord]);
                }
            }
        }
        //println!("{:#?}", self.map_by_counter);
    }

    pub fn sort_by_target(&mut self) {
        //  BTreeMap<u32, BTreeMap<String, Vec<CleanRecord>>>,
        let mut innerbtree: BTreeMap<String, Vec<CleanRecord>> = BTreeMap::new();

        // Loop through main map
        for (_kw, kw_meta) in self.map.iter_mut() {
            // BTreeMap<String, CleanRecordContainer>
            // Loop meta
            let cleanrecords: Vec<CleanRecord> = kw_meta.list.clone();
            // Then count target
            for record in cleanrecords.iter() {
                let target = record.target.clone();
                // if target
                if innerbtree.get(&target).is_none() {
                    // add to
                    let rerecord: CleanRecord = CleanRecord {
                        date_time: record.date_time.clone(),
                        keyword: record.keyword.clone(),
                        source: record.source.clone(),
                        hits: record.hits,
                        target: record.target.clone(),
                    };
                    innerbtree.insert(target.clone(), vec![rerecord]);
                } else {
                    let mut item = innerbtree.get(&target).unwrap();
                    item.1
                }
            }
        }
        println!("{:#?}", innerbtree);

        //
        // let mut target_holder: BTreeMap<String, u32> = BTreeMap::new();
        // // Loop through main map
        // for entry in self.map.iter_mut() {
        //     // Extract meta
        //     let kw_meta: CleanRecordContainer = entry.1.clone();
        //     let cleanrecord_container: Vec<CleanRecord> = kw_meta.list;
        //     // Then count target
        //     for record in cleanrecord_container.iter() {
        //         let target = record.target.clone();
        //         target_holder
        //             .entry(target)
        //             .and_modify(|count| *count += 1)
        //             .or_insert(1);
        //     }
        //
        //     for (target_link, counter) in target_holder.iter() {
        //         if self.map_by_target.get(&counter).is_none() {
        //             let mut btreeinner: BTreeMap<String, Vec<CleanRecord>> = BTreeMap::new();
        //             btreeinner.insert(target_link.clone(), cleanrecord_container.clone());
        //             self.map_by_counter.insert(counter.clone(), btreeinner);
        //         } else {
        //             // key (aka counter) exists, expand the vec
        //             let entry = self.map_by_target.get_mut(&counter).unwrap();
        //
        //             for cleanrecord in cleanrecord_container.iter() {
        //                 let newrecord: CleanRecord = CleanRecord {
        //                     date_time: cleanrecord.date_time.clone(),
        //                     keyword: cleanrecord.keyword.clone(),
        //                     source: cleanrecord.source.clone(),
        //                     hits: cleanrecord.hits,
        //                     target: cleanrecord.target.clone(),
        //                 };
        //                 entry.insert(cleanrecord.target.clone(), vec![newrecord]);
        //             }
        //         }
        //     }

        /*
        if self.map_by_counter.get(&counter).is_none() {
            let mut btreeinner: BTreeMap<String, Vec<CleanRecord>> = BTreeMap::new();
            btreeinner.insert(the_keyword.clone(), cleanrecord_container);
            self.map_by_counter.insert(counter, btreeinner);
        } else {
            // key (aka counter) exists, expand the vec
            let entry = self.map_by_counter.get_mut(&counter).unwrap();
            for cleanrecord in cleanrecord_container.iter() {
                let newrecord: CleanRecord = CleanRecord {
                    date_time: cleanrecord.date_time.clone(),
                    keyword: cleanrecord.keyword.clone(),
                    source: cleanrecord.source.clone(),
                    hits: cleanrecord.hits,
                    target: cleanrecord.target.clone(),
                };
                entry.insert(the_keyword.clone(), vec![newrecord]);
            }
        }*/
        //}
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

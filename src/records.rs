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
            STAT_TYPE::DOI => todo!(),
            STAT_TYPE::InvalidSearch => todo!(),
        }
    }

    pub fn show_stats(&self) {
        todo!();
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

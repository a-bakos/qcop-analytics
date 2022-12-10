use std::collections::HashMap;

#[derive(Debug)]
pub struct RecordCollection {
    pub map: HashMap<String, (u32, CleanRecordContainer)>,
}

impl RecordCollection {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
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
            values.0 = values.0 + 1;
        }
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

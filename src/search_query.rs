// "id","time","query","url","hits","user","email","temp_id","target","ip"
#[derive(Debug)]
pub struct SearchQuery {
    pub date_time: String,
    pub keyword: String,
    pub source: String,
    pub hits: u32,
    pub target: String,
}

impl SearchQuery {
    pub fn new(query: (String, String, String, u32, String)) -> Self {
        Self {
            date_time: query.0,
            keyword: query.1,
            source: query.2,
            hits: query.3,
            target: query.4,
        }
    }
}

pub fn default() -> String {
    String::from("")
}

pub fn source_url(url: &str) -> String {
    get_decoded_url(url)
}

fn get_decoded_url(url: &str) -> String {
    urlencoding::decode(url.trim()).unwrap().into_owned()
}

pub fn hits(hits: &str) -> String {
    // todo - return u32 instead?
    hits.trim().to_string()
}

pub fn target_url(url: &str) -> String {
    url.trim().to_string()
}

pub fn keyword(keyword: &str) -> String {
    keyword.trim().to_string()
}

pub fn datetime(datetime: &str) -> String {
    datetime.trim().to_string()
}

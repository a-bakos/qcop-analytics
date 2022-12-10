use crate::consts;

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
    let mut processed_kw = keyword.trim().to_string();

    // Find "+" in kw and replace it with whitespace
    processed_kw = processed_kw.replace('+', " ");

    if filter_invalid(keyword) {
        maybe_store_invalid_keyword(keyword);
        processed_kw = consts::KEYWORD_INVALID.to_string();
    }

    processed_kw
}

pub fn datetime(datetime: &str) -> String {
    datetime.trim().to_string()
}

pub fn is_valid_length(keyword: &str) -> bool {
    keyword.len() >= consts::KEYWORD_MIN_LENGTH && keyword.len() <= consts::KEYWORD_MAX_LENGTH
}

fn filter_invalid(keyword: &str) -> bool {
    for invalid_kw in consts::INVALID_KEYWORD_LIST.into_iter() {
        if invalid_kw == keyword {
            return true;
        }
    }
    false
}

// TODO!
fn maybe_store_invalid_keyword(keyword: &str) {
    if consts::STORE_INVALID_ITEMS {
        //we need a global storage for invalid keywords and we'll consume the parameter
    }
}

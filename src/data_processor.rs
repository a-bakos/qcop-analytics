use crate::consts;
use regex::Regex;

pub fn default() -> String {
    String::from("")
}

pub fn source_url(url: &str) -> String {
    get_decoded_url(url)
}

fn get_decoded_url(url: &str) -> String {
    urlencoding::decode(url.trim()).unwrap().into_owned()
}

pub fn hits(hits: &str) -> i32 {
    hits.trim()
        .parse::<i32>()
        .unwrap_or(consts::DEFAULT_MISSING_HITS)
}

pub fn target_url(url: &str) -> String {
    url.trim().to_string()
}

pub fn keyword(keyword: &str) -> String {
    let mut processed_kw = keyword.trim().to_string();

    // Find "+" in kw and replace it with whitespace
    processed_kw = processed_kw.replace('+', " ");

    if filter_known_invalid(keyword) {
        maybe_store_invalid_keyword(keyword);
        processed_kw = consts::DEFAULT_KEYWORD_INVALID.to_string();
        return processed_kw;
    }
    if filter_unknown_invalid(keyword) {
        maybe_store_invalid_keyword(keyword);
        processed_kw = consts::DEFAULT_KEYWORD_INVALID.to_string();
        return processed_kw;
    }

    if true == multi_filter(keyword) {
        processed_kw = consts::DEFAULT_KEYWORD_INVALID.to_string();
        return processed_kw;
    }

    processed_kw
}

fn multi_filter(keyword: &str) -> bool {
    let re = Regex::new(r"(?i)\b(and|like|or)\b.*\b(and|like|or)\b.*\b(and|like|or)\b").unwrap();
    if true == re.is_match(keyword) {
        println!("FOUND IT in here!: {:#?}", keyword);
        for cap in re.captures_iter(keyword) {
            if cap[1].to_lowercase() == cap[2].to_lowercase()
                && cap[2].to_lowercase() == cap[3].to_lowercase()
            {
                // All the same, might not be invalid
                return false;
            }
        }
        return true;
    }
    false
}

pub fn datetime(datetime: &str) -> String {
    datetime.trim().to_string()
}

pub fn is_valid_length(keyword: &str) -> bool {
    keyword.len() >= consts::KEYWORD_MIN_LENGTH && keyword.len() <= consts::KEYWORD_MAX_LENGTH
}

// To filter known, invalid keywords
fn filter_known_invalid(keyword: &str) -> bool {
    for invalid_kw in consts::INVALID_KEYWORD_LIST.into_iter() {
        if invalid_kw.to_lowercase() == keyword.to_lowercase() {
            return true;
        }
    }
    false
}

// To filter unknown, invalid "terms" such as SQL commands and any other non-sense attempts
fn filter_unknown_invalid(keyword: &str) -> bool {
    let mut match_found = false;
    for tainted_kw in consts::TAINTED_SEARCHES.into_iter() {
        match keyword.to_lowercase().find(tainted_kw) {
            Some(_) => {
                match_found = true;
                break;
            }
            None => (),
        }
    }
    match_found
}

// TODO!
fn maybe_store_invalid_keyword(keyword: &str) {
    if consts::STORE_INVALID_ITEMS {
        //we need a global storage for invalid keywords and we'll consume the parameter
    }
}

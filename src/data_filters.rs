// todo rename this file to data_filters

use crate::{consts, records};
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

pub fn clean_keyword(keyword: &str) -> String {
    let mut processed_kw = keyword.trim().to_string();
    processed_kw = processed_kw.to_lowercase();
    processed_kw = maybe_strip_elements(processed_kw);

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
    if multi_word_filter(keyword) {
        maybe_store_invalid_keyword(keyword);
        processed_kw = consts::DEFAULT_KEYWORD_INVALID.to_string();
        return processed_kw;
    }
    if multi_char_filter(keyword) {
        maybe_store_invalid_keyword(keyword);
        processed_kw = consts::DEFAULT_KEYWORD_INVALID.to_string();
        return processed_kw;
    }
    //if filter_nonlatin_chars(keyword) {
    //    processed_kw = consts::DEFAULT_KEYWORD_INVALID.to_string();
    //    return processed_kw;
    //}

    processed_kw.trim().to_string()
}

fn maybe_strip_elements(keyword: String) -> String {
    let mut processed_kw = keyword;

    processed_kw = processed_kw.replace('+', " ");
    processed_kw = processed_kw.replace("&amp;", "&");

    if processed_kw.contains("\\&quot;") {
        processed_kw = processed_kw.replace("\\&quot;", "");
    }
    if processed_kw.contains("\"") {
        processed_kw = processed_kw.replace("\"", "");
    }
    if processed_kw.contains(")") {
        processed_kw = processed_kw.replace(")", "");
    }
    if processed_kw.contains("(") {
        processed_kw = processed_kw.replace("(", "");
    }
    if processed_kw.contains("\"") {
        processed_kw = processed_kw.replace("\"", "");
    }
    if processed_kw.contains("_") {
        processed_kw = processed_kw.replace("_", " ");
    }
    if processed_kw.contains("‘") {
        processed_kw = processed_kw.replace("‘", "");
    }
    if processed_kw.contains("’") {
        processed_kw = processed_kw.replace("’", "");
    }
    if processed_kw.contains("`") {
        processed_kw = processed_kw.replace("`", "");
    }
    if processed_kw.contains("´") {
        processed_kw = processed_kw.replace("´", "");
    }
    if processed_kw.contains("•") {
        processed_kw = processed_kw.replace("•", "");
    }
    if processed_kw.contains("“") {
        processed_kw = processed_kw.replace("“", "");
    }
    if processed_kw.contains("”") {
        processed_kw = processed_kw.replace("”", "");
    }
    if processed_kw.contains("„") {
        processed_kw = processed_kw.replace("„", "");
    }

    /*if processed_kw.starts_with("\\&quot;") {
        processed_kw = processed_kw.strip_prefix("\\&quot;").unwrap().to_string();
    }
    if processed_kw.ends_with("\\&quot;") {
        processed_kw = processed_kw.strip_suffix("\\&quot;").unwrap().to_string();
    }*/
    processed_kw
}

pub fn handle_if_meaningful(keyword: &str, collection: &mut records::RecordCollection) -> String {
    handle_if_search_is_doi(keyword, collection);

    keyword.to_string()
}

fn handle_if_search_is_doi(keyword: &str, collection: &mut records::RecordCollection) -> bool {
    // DOI's start with 10.xxxx, where x = a digit
    match keyword.to_lowercase().find("10.") {
        Some(_) => {
            let re = Regex::new(r"(?i)10.\d{4}").unwrap();
            if re.is_match(keyword) {
                //println!("Found DOI search! {:#?}", keyword);
                collection.add_to_stats(records::StatType::DOI);
                return true;
            }
            false
        }
        None => false,
    }
}

fn multi_word_filter(keyword: &str) -> bool {
    let re = Regex::new(r"(?i)\b(and|like|or)\b.*\b(and|like|or)\b.*\b(and|like|or)\b").unwrap();
    if re.is_match(keyword) {
        //println!("FOUND IT in here!: {:#?}", keyword);
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

fn multi_char_filter(keyword: &str) -> bool {
    let re = Regex::new(r"(?i)%.*%").unwrap();
    if re.is_match(keyword) {
        return true;
    }
    false
}

fn filter_nonlatin_chars(keyword: &str) -> bool {
    // Unicode property escape:
    // Exclude every non-Latin character, but allow digits, punctuation, and some other characters
    let re = Regex::new(r"^[\p{L}0-9 .:;\\/,!?-]*$").unwrap();
    if re.is_match(keyword) {
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

pub fn is_numeric_only(keyword: &str) -> bool {
    match keyword.trim().parse::<f64>() {
        Ok(_) => true,
        _ => false,
    }
}

// To filter known, invalid keywords
fn filter_known_invalid(keyword: &str) -> bool {
    for invalid_kw in consts::INVALID_KEYWORD_LIST.into_iter() {
        if invalid_kw.to_lowercase() == keyword.to_lowercase() {
            return true;
        }
    }

    for invalid_kw_start in consts::INVALID_KEYWORD_START_LIST.into_iter() {
        if keyword.starts_with(invalid_kw_start) {
            return true;
        }
    }
    false
}

// To filter unknown, invalid "terms" such as SQL commands and any other non-sense attempts
fn filter_unknown_invalid(keyword: &str) -> bool {
    let mut match_found = false;
    for tainted_kw in consts::TAINTED_SEARCHES.into_iter() {
        if keyword.to_lowercase().contains(tainted_kw) {
            match_found = true;
            break;
        }
    }
    match_found
}

// TODO!
fn maybe_store_invalid_keyword(_keyword: &str) {
    if consts::STORE_INVALID_ITEMS {
        //we need a global storage for invalid keywords and we'll consume the parameter
    }
}

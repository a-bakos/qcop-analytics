/// Config parameters
pub const KEYWORD_MAX_LENGTH: usize = 100;
pub const KEYWORD_MIN_LENGTH: usize = 3;
pub const EXCLUDE_LOGGED_IN_USER_SEARCHES: bool = true;
pub const STORE_INVALID_ITEMS: bool = true;

/// Invalid KW means when a search happened but the keyword couldn't be
/// recorded for some reason.
pub const INVALID_KEYWORD_LIST: [&str; 1] = ["{search_term_string}"];

/// Define characters and keywords here that would make a search entry tainted
/// if present. Keep it lowercase.
/// This list could come from an external file, but it is intentionally baked
/// into the binary at the moment.
pub const TAINTED_SEARCHES: [&str; 3] = ["(select", "varchar", "|"];

// DEV values
pub const CSV_INPUT_FILE_NAME: &str = "export/test.csv"; //"export/as-wp_q_cop.csv";
pub const CSV_OUTPUT_FILE_NAME: &str = "outtest.csv";

/// Defaults

/// Used internally after a KW has been treated by the program
pub const DEFAULT_KEYWORD_INVALID: &str = "{invalid_keyword}";

pub const DEFAULT_MISSING_HITS: i32 = -1; // could be i16

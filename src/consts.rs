/// Config parameters
pub const KEYWORD_MAX_LENGTH: usize = 100;
pub const KEYWORD_MIN_LENGTH: usize = 3;
pub const EXCLUDE_LOGGED_IN_USER_SEARCHES: bool = true;
pub const EXCLUDE_ONLY_NUMBER_SEARCHES: bool = true;
pub const STORE_INVALID_ITEMS: bool = true;
pub const NUMBER_OF_TOP_KEYWORDS: usize = 200;
pub const NUMBER_OF_TOP_TARGETS: usize = 10;

/// Invalid KW means when a search happened but the keyword couldn't be
/// recorded for some reason.
pub const INVALID_KEYWORD_LIST: [&str; 1] = ["{search_term_string}"];

pub const INVALID_KEYWORD_START_LIST: [&str; 6] = ["1/", "1*", "1\\", "1%", "1&", "1$"];

// TODO - will be used for kw that we specifically look for
pub const SPECIAL_KEYWORD_LIST: [&str; 0] = [];

// DEV values
// pub const CSV_INPUT_FILE_NAME: &str = "import/test.csv";
pub const CSV_INPUT_FILE_NAME: &str = "import/as-wp_q_cop.csv";
// pub const CSV_INPUT_FILE_NAME: &str = "import/eec_q_cop.csv";
// pub const CSV_INPUT_FILE_NAME: &str = "import/er-wp_q_cop.csv";
// pub const CSV_INPUT_FILE_NAME: &str = "import/lr-tandfq_cop.csv";
// pub const CSV_INPUT_FILE_NAME: &str = "import/nr-wp_q_cop.csv";

pub const CSV_OUTPUT_FILE_NAME: &str = "export/collection.csv";

// Ordered files
pub const CSV_OUTPUT_FILE_NAME_ORDER_BY_AZ: &str = "export/collection-order-by-az.csv";
pub const CSV_OUTPUT_FILE_NAME_ORDER_BY_COUNT: &str = "export/collection-order-by-count.csv";
pub const CSV_OUTPUT_FILE_NAME_ORDER_BY_SOURCE: &str = "export/collection-order-by-source.csv";
pub const CSV_OUTPUT_FILE_NAME_ORDER_BY_TARGET: &str = "export/collection-order-by-target.csv";
pub const CSV_OUTPUT_FILE_NAME_TOP_KEYWORDS: &str = "export/collection-top-keywords.csv";

pub const CSV_COLUMN_INDEX_ID: usize = 0;
pub const CSV_COLUMN_INDEX_DATETIME: usize = 1;
pub const CSV_COLUMN_INDEX_QUERY: usize = 2;
pub const CSV_COLUMN_INDEX_URL: usize = 3;
pub const CSV_COLUMN_INDEX_HITS: usize = 4;
pub const CSV_COLUMN_INDEX_USER: usize = 5;
pub const CSV_COLUMN_INDEX_EMAIL: usize = 6;
pub const CSV_COLUMN_INDEX_TEMP_ID: usize = 7;
pub const CSV_COLUMN_INDEX_TARGET: usize = 8;
pub const CSV_COLUMN_INDEX_IP: usize = 9;

/// Defaults

/// Used internally after a KW has been treated by the program
pub const DEFAULT_KEYWORD_INVALID: &str = "{invalid_keyword}";

pub const DEFAULT_MISSING_HITS: i32 = -1; // could be i16

/// Stat type variations - may need different format later
pub const STAT_DOI: &str = "doi";
pub const STAT_INVALID: &str = "invalid";

/// Define characters and keywords here that would make a search entry tainted
/// if present. Keep it lowercase.
/// This list could come from an external file, but it is intentionally baked
/// into the binary at the moment.
pub const TAINTED_SEARCHES: [&str; 101] = [
    "&lt;",
    "&gt;",
    "1.",
    "1#",
    "x_",
    "#{",
    "$%",
    "search_term_string",
    ".php",
    ".html",
    "=",
    "--",
    "(select",
    "select/",
    "varchar",
    "|",
    "/feed",
    "feed/",
    "/page",
    "/and",
    "__import__",
    "waitfor",
    "delay--",
    "xmlrpc",
    "and--",
    "or--",
    "app/",
    "invokefunction",
    "/script",
    "script/",
    ":alert",
    "\\u",
    "2000\\",
    "2000/",
    "2000&",
    "2000*",
    "*/",
    "&#",
    ";;",
    ",,",
    "..",
    "ｅ",
    "ｓ",
    "\\\\",
    "javascript",
    "http",
    ".doc",
    ".xml",
    ".asp",
    ".pdf",
    ".jpg",
    ".jpeg",
    "*pdf",
    "file:",
    "dbms",
    "web-inf",
    "web-console",
    "web.config",
    "/index",
    "\\index",
    "index/",
    "index\\",
    "..;/",
    "/common/",
    "ptst",
    "win.ini",
    "\\&#039;",
    "gtm.start",
    "appcheck",
    ".zip",
    ".rar",
    ".exe",
    ";foo",
    "/home",
    "()",
    ";sleep",
    "sleep(",
    "printf",
    "{",
    "}",
    "alert(",
    "document.cookie",
    "intval(",
    "./",
    ".\\",
    "...",
    ".bash_",
    ".ens",
    ".env",
    ".yml",
    ".wav",
    "passwd",
    "fuck",
    ".git",
    "192.",
    "@",
    "trackingid",
    "www",
    "xxx",
    ".com",
    "zw5",
];

pub const TAINTED_TARGETS: [&str; 5] = [
    "cdn-cgi",
    "admin-ajax",
    "jAvasCriPt%",
    ".text-center",
    "class=",
];

pub const STRIP_FROM_ENTRY: [&str; 2] = ["\"", "\\&quot;"];

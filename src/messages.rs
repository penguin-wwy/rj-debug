pub const GET_METHOD_NAME_ERROR: &'static str = "Get method name error";
pub const GET_LOCAL_VARIABLE_ERROR: &'static str = "Get local variable table failed";
pub const GET_LINE_TABLE_ERROR: &'static str = "Get line number table failed";
pub const GET_CLASS_SIGNATURE_ERROR: &'static str = "Get class signature failed";
pub const UNKNOWN_BREAKPOINT: &'static str = "Unknown breakpoint";
pub const SET_BREAKPOINT_ERROR: &'static str = "Set breakpoint failed";
pub const JSON_FILE_PARSE_ERROR: &'static str = "File parse json error";

pub const METHOD_SIGNATURE_ERROR: &'static str = "method signature format error";

pub fn error_with_set_event(event: &str) -> String {
    format!("Set {} event failed", event)
}

pub fn message_by_file_parse(message: &str, file: &str) -> String {
    format!("{} [file: {}]", message, file)
}

pub fn message_with_path(message: &str, path: &str) -> String {
    format!("{} [address: {}]", message, path)
}
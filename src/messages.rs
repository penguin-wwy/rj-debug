pub const GET_METHOD_NAME_ERROR: &'static str = "Get method name error...";

pub const METHOD_SIGNATURE_ERROR: &'static str = "method signature format error";

pub fn message_with_path(message: &str, path: &str) -> String {
    format!("{} [address: {}]", message, path)
}
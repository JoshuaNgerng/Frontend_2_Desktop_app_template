use std::borrow::Cow;
use convert_case::Case;

pub fn ts_to_rust_type(ts_type: &str) -> Cow<'_, str> {
    Cow::Borrowed(match ts_type.trim() {
        // primitives
        "string" => "String",
        "number" => "f64",
        "boolean" => "bool",
        "bigint" => "i64",

        // date/time
        "Date" => "chrono::DateTime<chrono::Utc>",
        "ISOString" => "chrono::DateTime<chrono::Utc>",
        "Timestamp" => "chrono::DateTime<chrono::Utc>",

        // arrays
        "string[]" => "Vec<String>",
        "number[]" => "Vec<f64>",
        "boolean[]" => "Vec<bool>",

        // generic fallback
        _ => return Cow::Owned(check_ts_array_syntax(ts_type)),
    })
}

fn check_ts_array_syntax(ts_type: &str) -> String {
    if ts_type.ends_with("[]") {
        let new_len = ts_type.chars().count() - 2;
        let s: String = ts_type.chars().take(new_len).collect();
        format!("Vec<{}>", s)
    } else {
        ts_type.to_string()
    }
}

pub fn infer_enum_case(covert_case: &str) -> Option<Case<'static>> {
    Some(match covert_case {
        "Camel" => convert_case::Case::Camel,
        "Snake" => convert_case::Case::Snake,
        "Pascal" => convert_case::Case::Pascal,
        "Upper" => convert_case::Case::UpperFlat,
        "Constant" => convert_case::Case::Constant,
        "Title" => convert_case::Case::Title,
        "Train" => convert_case::Case::Train,
        _ => return None
    })
}

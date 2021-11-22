use crate::core::util::EmptyOrWhitespace;
use gmime;

pub enum DispositionType {
    Unspecified,
    Attachment,
    Inline,
    Other(String),
}

impl DispositionType {
    pub fn deserialize(ser: &str) -> Self {
        if ser.is_empty_or_whitespace() {
            return DispositionType::Unspecified;
        }
        match ser.to_ascii_lowercase().as_ref() {
            "inline" => DispositionType::Inline,

            "attachment" => DispositionType::Attachment,
            unknown => DispositionType::Other(unknown.to_owned()),
        }
    }
}

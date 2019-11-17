#[derive(Debug)]
pub enum DeserialiseError {
    QuickXMLError(::quick_xml::Error),
    Utf8Error(::std::str::Utf8Error),
    ParseIntError(::std::num::ParseIntError),
    ParseFloatError(::std::num::ParseFloatError),
    Error(String),
    TextNotFound(String),
    TagNotRecognised(String),
    TagNotFound(String),
}

impl DeserialiseError {
    pub fn error(error_message: &str) -> Self {
        DeserialiseError::Error(format!("{}", error_message))
    }

    pub fn text_not_found() -> Self {
        DeserialiseError::TextNotFound(format!("Text not found"))
    }

    pub fn tag_not_recognised(tag_name: &str) -> Self {
        DeserialiseError::TagNotRecognised(format!("Tag no recognised: {}", tag_name))
    }

    pub fn tag_not_found(tag_name: &str) -> Self {
        DeserialiseError::TagNotFound(format!("Tag not found: {}", tag_name))
    }
}

impl From<quick_xml::Error> for DeserialiseError {
    fn from(error: ::quick_xml::Error) -> DeserialiseError {
        DeserialiseError::QuickXMLError(error)
    }
}

impl From<::std::str::Utf8Error> for DeserialiseError {
    fn from(error: ::std::str::Utf8Error) -> DeserialiseError {
        DeserialiseError::Utf8Error(error)
    }
}

impl From<::std::num::ParseIntError> for DeserialiseError {
    fn from(error: ::std::num::ParseIntError) -> DeserialiseError {
        DeserialiseError::ParseIntError(error)
    }
}

impl From<::std::num::ParseFloatError> for DeserialiseError {
    fn from(error: ::std::num::ParseFloatError) -> DeserialiseError {
        DeserialiseError::ParseFloatError(error)
    }
}

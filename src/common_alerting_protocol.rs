pub mod alert;
pub mod area;
pub mod circle;
pub mod event_code;
pub mod geocode;
pub mod info;
pub mod parameter;
pub mod point;
pub mod polygon;
pub mod resource;
pub mod utilities;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ParseEnumError {
    enum_string: String,
}

impl ParseEnumError {
    pub fn enum_not_found(enum_string: &str) -> ParseEnumError {
        ParseEnumError {
            enum_string: String::from(enum_string),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    QuickXMLError(::quick_xml::Error),
    Utf8Error(::std::str::Utf8Error),
    ParseIntError(::std::num::ParseIntError),
    ParseFloatError(::std::num::ParseFloatError),
    ParseDatError(chrono::ParseError),
    Error(String),
    TextNotFound(String),
    TagNotRecognised(String),
    TagNotFound(String),
    TagNotExpected(String),
    ParseEnumError(ParseEnumError),
    EofReached,
    UnknownEvent(String),
    NameSpaceNotFound,
}

impl Error {
    pub fn error(error_message: &str) -> Self {
        Error::Error(format!("{}", error_message))
    }

    pub fn text_not_found(error_message: &str) -> Self {
        Error::TextNotFound(format!("Text not found: {}", error_message))
    }

    pub fn tag_not_recognised(tag_name: &str) -> Self {
        Error::TagNotRecognised(format!("Tag not recognised: {}", tag_name))
    }

    pub fn tag_not_found(tag_name: &str) -> Self {
        Error::TagNotFound(format!("Tag not found: {}", tag_name))
    }

    pub fn tag_not_expected(tag_name: &str) -> Self {
        Error::TagNotExpected(format!("Tag not expected: {}", tag_name))
    }

    pub fn enum_not_found(expected_enum: &str) -> Self {
        Error::ParseEnumError(ParseEnumError::enum_not_found(expected_enum))
    }

    pub fn unknown_event(event: ::quick_xml::events::Event) -> Self {
        Error::UnknownEvent(format!("{:?}", event))
    }
}

impl From<quick_xml::Error> for Error {
    fn from(error: ::quick_xml::Error) -> Error {
        Error::QuickXMLError(error)
    }
}

impl From<::std::str::Utf8Error> for Error {
    fn from(error: ::std::str::Utf8Error) -> Error {
        Error::Utf8Error(error)
    }
}

impl From<::std::num::ParseIntError> for Error {
    fn from(error: ::std::num::ParseIntError) -> Error {
        Error::ParseIntError(error)
    }
}

impl From<::std::num::ParseFloatError> for Error {
    fn from(error: ::std::num::ParseFloatError) -> Error {
        Error::ParseFloatError(error)
    }
}
impl From<ParseEnumError> for Error {
    fn from(error: ParseEnumError) -> Error {
        Error::ParseEnumError(error)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Error {
        Error::ParseDatError(error)
    }
}

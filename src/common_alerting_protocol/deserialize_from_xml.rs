use quick_xml::Reader;

#[derive(Debug)]
pub enum DeserialiseError {
    QuickXMLError(::quick_xml::Error),
    Utf8Error(::std::str::Utf8Error),
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

pub trait DeserializeFromXml {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Box<Self>, DeserialiseError>;
}

use quick_xml::Reader;

#[derive(Debug)]
pub enum SerializeError {
    Error(String),
    TextNotFound(String),
    TagNotRecognised(String),
    TagNotFound(String),
}

pub trait DeserializeFromXml {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Box<Self>, SerializeError>;
}

use quick_xml::Reader;

#[derive(Debug)]
pub enum SerializeError {
    Error(&'static str),
    TextNotFound(&'static str),
    TagNotRecognised(&'static str)
}

pub trait DeserializeFromXml {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Box<Self>, SerializeError>;
}

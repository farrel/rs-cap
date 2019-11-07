use quick_xml::Reader;

pub trait DeserializeFromXml {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Option<Box<Self>>;
}

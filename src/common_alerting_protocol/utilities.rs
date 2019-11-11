use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialize_from_xml::SerializeError;

pub fn parse_string(reader: &mut Reader<&[u8]>) -> Result<String, SerializeError> {
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let mut name_string = String::new();

    match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
        Ok((_, Event::Text(name))) => {
            name_string.push_str(&name.unescape_and_decode(reader).unwrap());
        }
        _ => return Err(SerializeError::TextNotFound("Expecting string")),
    }

    return Ok(name_string);
}

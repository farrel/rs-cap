use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

const NAME_TAG: &[u8] = b"valueName";
const VALUE_TAG: &[u8] = b"value";
const EVENT_CODE_TAG: &[u8] = b"eventCode";

static EVENT_CODE: &str = "eventCode";

pub struct EventCode {
    name: String,
    value: String,
}

//fn get_name(reader: &mut Reader<&[u8]>, namespace: &[u8]) -> Result<String, DeserialiseError> {
//    println!("GET NAME");
//    let name = parse_string(reader)?;
//    parse_end_tag(reader, NAME_TAG, namespace)?;
//    return Ok(name);
//}
//
//fn get_value(reader: &mut Reader<&[u8]>, namespace: &[u8]) -> Result<String, DeserialiseError> {
//    println!("GET VALUE");
//    let value = parse_string(reader)?;
//    parse_end_tag(reader, VALUE_TAG, namespace)?;
//    return Ok(value);
//}

impl EventCode {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<EventCode, DeserialiseError> {
        let mut event_code = EventCode {
            name: String::new(),
            value: String::new(),
        };

        loop {
            buf.clear();
            match reader.read_namespaced_event(buf, ns_buf) {
                Ok((_, Event::Eof)) => return Err(DeserialiseError::error("Unexpected EOF")),
                Ok((Some(ref namespace), Event::Start(ref e))) => match e.local_name() {
                    NAME_TAG => {
                        event_code.name = reader.read_text(NAME_TAG, &mut Vec::new())?;
                    }
                    VALUE_TAG => {
                        event_code.value = reader.read_text(VALUE_TAG, &mut Vec::new())?;
                    }
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(str::from_utf8(unknown_tag)?)),
                },
                Ok((Some(ref namespace), Event::End(ref e))) => match e.local_name() {
                    EVENT_CODE_TAG => return Ok(event_code),
                    NAME_TAG | VALUE_TAG => (),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(str::from_utf8(unknown_tag)?)),
                },
                Ok((Some(ref namespace), ref event)) => {
                    println!("EVENT: {:?} NS: {:?}", event, str::from_utf8(namespace).unwrap());
                }
                Ok((None, event)) => return Err(DeserialiseError::unknown_event(event)),
                Err(e) => return Err(DeserialiseError::from(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
    use crate::common_alerting_protocol::event_code::EventCode;
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use std::str;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<eventCode xmlns="urn:oasis:names:tc:emergency:cap:1.2"><valueName>Name</valueName><value>Value</value></eventCode>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);
        let event_code = EventCode::deserialize_from_xml(VERSION_1_2, reader, &mut buf, &mut ns_buf).unwrap();

        assert_eq!("Name", event_code.name);
        assert_eq!("Value", event_code.value);
    }
}

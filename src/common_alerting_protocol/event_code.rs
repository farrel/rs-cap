use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

const NAME_TAG: &[u8] = b"valueName";
const VALUE_TAG: &[u8] = b"value";
const EVENT_CODE_TAG: &[u8] = b"eventCode";

pub struct EventCode {
    name: String,
    value: String,
}

impl EventCode {
    pub fn deserialize_from_xml(namespace: &[u8], reader: &mut Reader<&[u8]>) -> Result<EventCode, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        let mut event_code = EventCode {
            name: String::new(),
            value: String::new(),
        };

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (ref ns, Event::Start(ref e)) => match (*ns, e.local_name()) {
                    (Some(namespace), EVENT_CODE_TAG) => (),
                    (Some(namespace), NAME_TAG) => event_code.name.push_str(&parse_string(reader, NAME_TAG)?),
                    (Some(namespace), VALUE_TAG) => event_code.value.push_str(&parse_string(reader, VALUE_TAG)?),
                    (_, unknown_tag) => return Err(DeserialiseError::tag_not_recognised(str::from_utf8(unknown_tag)?)),
                },
                (ref ns, Event::End(ref e)) => match (*ns, e.local_name()) {
                    (Some(namespace), EVENT_CODE_TAG) => return Ok(event_code),
                    (_, unknown_tag) => return Err(DeserialiseError::tag_not_recognised(str::from_utf8(unknown_tag)?)),
                },
                (_, Event::Eof) => panic!("NO"),
                _ => (),
            }
            buf.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
    use crate::common_alerting_protocol::event_code::EventCode;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<eventCode xmlns="urn:oasis:names:tc:emergency:cap:1.2">
                        <valueName>Name</valueName>
                        <value>Value</value>
                    </eventCode>"#;

        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        let event_code = EventCode::deserialize_from_xml(VERSION_1_2, reader).unwrap();

        assert_eq!("Name", event_code.name);
        assert_eq!("Value", event_code.value);
    }

    #[test]
    fn test_from_docs() {
        use quick_xml::events::Event;
        use quick_xml::Reader;
        use std::str::from_utf8;

        let xml = r#"<x:tag1 xmlns:x="www.xxxx" xmlns:y="www.yyyy" att1 = "test">
                <y:tag2><!--Test comment-->Test</y:tag2>
                <y:tag2>Test 2</y:tag2>
            </x:tag1>"#;
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut count = 0;
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let mut txt = Vec::new();
        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
                Ok((ref ns, Event::Start(ref e))) => {
                    count += 1;
                    match (*ns, e.local_name()) {
                        (Some(b"www.xxxx"), b"tag1") => (),
                        (Some(b"www.yyyy"), b"tag2") => (),
                        (ns, n) => panic!("Namespace and local name mismatch"),
                    }
                    println!("Resolved namespace: {:?}", ns.and_then(|ns| from_utf8(ns).ok()));
                }
                Ok((_, Event::Text(e))) => txt.push(e.unescape_and_decode(&reader).expect("Error!")),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok((_, Event::Eof)) => break,
                _ => (),
            }
            buf.clear();
        }
        println!("Found {} start events", count);
        println!("Text events: {:?}", txt);
    }
}

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;

use quick_xml::Reader;
use std::str;

const EVENT_CODE_TAG: &[u8] = b"eventCode";
static EVENT_CODE: &str = "eventCode";

pub struct EventCode {
    name: String,
    value: String,
}

impl EventCode {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<EventCode, DeserialiseError> {
        let (name, value) = parse_name_value_pair(reader, namespace, EVENT_CODE_TAG, buf, ns_buf)?;

        return Ok(EventCode { name: name, value: value });
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::{VERSION_1_2};
    use crate::common_alerting_protocol::event_code::EventCode;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<eventCode xmlns="urn:oasis:names:tc:emergency:cap:1.2"><valueName>Name</valueName><value>Value</value></eventCode>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);
        let event_code = EventCode::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();

        assert_eq!("Name", event_code.name);
        assert_eq!("Value", event_code.value);
    }

    #[test]
    fn test_deserialize_from_xml_wrong_ns() {
        let xml = r#"<eventCode xmlns="urn:oasis:names:tc:emergency:cap:1.1"><valueName>Name</valueName><value>Value</value></eventCode>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);
        match EventCode::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf) {
            Ok(_event_code) => panic!("Should not return Ok"),
            Err(_error) => (),
        }
    }
}

use crate::common_alerting_protocol::utilities::parse_name_value_pair;
use crate::common_alerting_protocol::Result;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::str;

const EVENT_CODE_TAG: &[u8] = b"eventCode";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventCode {
    pub name: Option<String>,
    pub value: Option<String>,
}

impl EventCode {
    pub fn initialise() -> EventCode {
        EventCode { name: None, value: None }
    }

    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<EventCode> {
        let (name, value) = parse_name_value_pair(reader, namespace, EVENT_CODE_TAG, buf, ns_buf)?;

        Ok(EventCode { name, value })
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::VERSION_1_2;
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

        assert_eq!(Some(String::from("Name")), event_code.name);
        assert_eq!(Some(String::from("Value")), event_code.value);
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

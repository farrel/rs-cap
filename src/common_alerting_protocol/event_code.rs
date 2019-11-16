use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialize_from_xml::{DeserialiseError, DeserializeFromXml};
use crate::common_alerting_protocol::utilities::*;

const NAME_TAG: &str = "valueName";
const VALUE_TAG: &str = "value";
const EVENT_CODE_TAG: &str = "eventCode";

pub struct EventCode {
    name: String,
    value: String,
}

impl DeserializeFromXml for EventCode {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Box<EventCode>, DeserialiseError> {
        let mut text = String::new();
        let mut name = String::new();
        let mut value = String::new();

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
                Ok((ref _ns, Event::Start(ref e))) => match str::from_utf8(e.name())? {
                    EVENT_CODE_TAG | NAME_TAG | VALUE_TAG => (),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },

                Ok((_ns, Event::Text(e))) => text.push_str(&e.unescape_and_decode(reader)?),

                Ok((_ns, Event::End(ref e))) => match str::from_utf8(e.name())? {
                    NAME_TAG => {
                        name.push_str(&text);
                        text.clear()
                    }
                    VALUE_TAG => {
                        value.push_str(&text);
                        text.clear()
                    }
                    EVENT_CODE_TAG => return Ok(Box::new(EventCode { name: name, value: value })),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::deserialize_from_xml::DeserializeFromXml;
    use crate::common_alerting_protocol::event_code::EventCode;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<eventCode>
                        <valueName>Name</valueName>
                        <value>Value</value>
                    </eventCode>"#;

        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        let event_code = EventCode::deserialize_from_xml(reader).unwrap();

        assert_eq!("Name", event_code.name);
        assert_eq!("Value", event_code.value);
    }
}

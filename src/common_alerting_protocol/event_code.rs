use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

const NAME_TAG: &str = "valueName";
const VALUE_TAG: &str = "value";
const EVENT_CODE_TAG: &str = "eventCode";

pub struct EventCode {
    name: String,
    value: String,
}

impl EventCode {
    pub fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<EventCode, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        let mut event_code = EventCode {
            name: String::new(),
            value: String::new(),
        };

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (ref _ns, Event::Start(ref e)) => match str::from_utf8(e.name())? {
                    EVENT_CODE_TAG => (),
                    NAME_TAG => event_code.name.push_str(&parse_string(reader, NAME_TAG)?),
                    VALUE_TAG => event_code.value.push_str(&parse_string(reader, VALUE_TAG)?),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                (ref _ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                    EVENT_CODE_TAG => return Ok(event_code),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
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

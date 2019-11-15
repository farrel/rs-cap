use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialize_from_xml::{DeserializeFromXml, SerializeError};
use crate::common_alerting_protocol::utilities::*;

const NAME_TAG: &str = "valueName";
const VALUE_TAG: &str = "value";
const EVENT_CODE_TAG: &str = "eventCode";

pub struct EventCode {
    name: String,
    value: String,
}

fn parse_name(reader: &mut Reader<&[u8]>) -> Result<String, SerializeError> {
    return Ok(parse_string(reader)?);
}

fn parse_value(reader: &mut Reader<&[u8]>) -> Result<String, SerializeError> {
    return Ok(parse_string(reader)?);
}

impl DeserializeFromXml for EventCode {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Box<EventCode>, SerializeError> {
        let mut name = String::new();
        let mut value = String::new();

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
                Ok((ref ns, Event::Start(ref e))) => {
                    let tag = str::from_utf8(e.name()).unwrap();
                    match tag {
                        EVENT_CODE_TAG => (),
                        NAME_TAG => name.push_str(&parse_name(reader)?),
                        VALUE_TAG => value.push_str(&parse_value(reader)?),
                        _ => {
                            return Err(SerializeError::TagNotRecognised(format!(
                                "Not expecting {} tag",
                                tag
                            )))
                        }
                    }
                }
                Ok((ref ns, Event::End(ref e))) => match str::from_utf8(e.name()).unwrap() {
                    EVENT_CODE_TAG => {
                        return Ok(Box::new(EventCode {
                            name: name,
                            value: value,
                        }))
                    }
                    _ => (),
                },
                _ => {
                    return Err(SerializeError::TagNotFound(
                        "Expecting to encounter tag".to_string(),
                    ))
                }
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

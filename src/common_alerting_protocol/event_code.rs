use crate::common_alerting_protocol::deserialize_from_xml::DeserializeFromXml;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::str;

const NAME_TAG: &str = "valueName";
const VALUE_TAG: &str = "value";
const EVENT_CODE_TAG: &str = "eventCode";

pub struct EventCode {
    name: String,
    value: String,
}

fn parse_name(reader: &mut Reader<&[u8]>) -> String {
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let mut name_string = String::new();

    match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
        Ok((_, Event::Text(name))) => {
            name_string.push_str(&name.unescape_and_decode(reader).unwrap());
        }
        _ => (),
    }

    return name_string;
}

fn parse_value(reader: &mut Reader<&[u8]>) -> String {
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let mut value_string = String::new();

    match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
        Ok((_, Event::Text(value))) => {
            value_string.push_str(&value.unescape_and_decode(reader).unwrap());
        }
        _ => (),
    }

    return value_string;
}

impl DeserializeFromXml for EventCode {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Option<Box<EventCode>> {
        let mut attributes: HashMap<&str, String> = HashMap::new();

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let mut tag_text = String::new();

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
                Ok((ref ns, Event::Start(ref e))) => match str::from_utf8(e.name()).unwrap() {
                    NAME_TAG => {
                        attributes.insert(NAME_TAG, parse_name(reader));
                    }
                    VALUE_TAG => {
                        attributes.insert(VALUE_TAG, parse_value(reader));
                    }
                    _ => (),
                },
                Ok((ref ns, Event::End(ref e))) => match str::from_utf8(e.name()).unwrap() {
                    EVENT_CODE_TAG => {
                        return Some(Box::new(EventCode {
                            name: attributes.get(NAME_TAG).unwrap().to_string(),
                            value: attributes.get(VALUE_TAG).unwrap().to_string(),
                        }));
                    }
                    _ => (),
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

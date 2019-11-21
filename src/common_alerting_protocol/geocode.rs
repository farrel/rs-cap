use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;

const NAME_TAG: &str = "valueName";
const VALUE_TAG: &str = "value";
pub const GEOCODE_TAG: &str = "geocode";

pub struct Geocode {
    name: String,
    value: String,
}

impl Geocode {
    pub fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Geocode, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        let mut geocode = Geocode {
            name: String::new(),
            value: String::new(),
        };

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (ref _ns, Event::Start(ref e)) => match str::from_utf8(e.name())? {
                    GEOCODE_TAG => (),
                    NAME_TAG => geocode.name.push_str(&parse_string(reader, NAME_TAG)?),
                    VALUE_TAG => geocode.value.push_str(&parse_string(reader, VALUE_TAG)?),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },

                (ref _ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                    GEOCODE_TAG => return Ok(geocode),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::geocode::Geocode;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<geocode>
                        <valueName>Name</valueName>
                        <value>Value</value>
                    </geocode>"#;

        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        let geocode = Geocode::deserialize_from_xml(reader).unwrap();

        assert_eq!("Name", geocode.name);
        assert_eq!("Value", geocode.value);
    }
}

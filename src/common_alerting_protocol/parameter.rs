use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

const NAME_TAG: &str = "valueName";
const VALUE_TAG: &str = "value";
pub const PARAMETER_TAG: &str = "parameter";

pub struct Parameter {
    name: String,
    value: String,
}

impl Parameter {
    pub fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Parameter, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        let mut parameter = Parameter {
            name: String::new(),
            value: String::new(),
        };

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (ref _ns, Event::Start(ref e)) => match str::from_utf8(e.name())? {
                    PARAMETER_TAG => (),
                    NAME_TAG => parameter.name.push_str(&parse_string(reader, NAME_TAG)?),
                    VALUE_TAG => parameter.value.push_str(&parse_string(reader, VALUE_TAG)?),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                (ref _ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                    PARAMETER_TAG => return Ok(parameter),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::parameter::Parameter;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<parameter>
                        <valueName>Name</valueName>
                        <value>Value</value>
                    </parameter>"#;

        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        let parameter = Parameter::deserialize_from_xml(reader).unwrap();

        assert_eq!("Name", parameter.name);
        assert_eq!("Value", parameter.value);
    }
}

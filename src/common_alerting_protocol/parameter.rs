use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

pub const PARAMETER_TAG: &[u8] = b"parameter";

pub struct Parameter {
    name: String,
    value: String,
}

impl Parameter {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Parameter, DeserialiseError> {
        let (name, value) = parse_name_value_pair(reader, namespace, PARAMETER_TAG, buf, ns_buf)?;

        return Ok(Parameter { name: name, value: value });
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
    use crate::common_alerting_protocol::parameter::Parameter;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<parameter xmlns="urn:oasis:names:tc:emergency:cap:1.2">
                         <valueName>Name</valueName>
                         <value>Value</value>
                     </parameter>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);

        let parameter = Parameter::deserialize_from_xml(VERSION_1_2, reader, &mut buf, &mut ns_buf).unwrap();

        assert_eq!("Name", parameter.name);
        assert_eq!("Value", parameter.value);
    }
}

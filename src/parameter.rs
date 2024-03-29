use crate::result::Result;
use crate::utilities::parse_name_value_pair;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

pub const PARAMETER_TAG: &[u8] = b"parameter";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Parameter {
    pub name: Option<String>,
    pub value: Option<String>,
}

impl Parameter {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Parameter> {
        let (name, value) = parse_name_value_pair(reader, namespace, PARAMETER_TAG, buf, ns_buf)?;

        Ok(Parameter { name, value })
    }
}

#[cfg(test)]
mod tests {
    use crate::alert::VERSION_1_2;
    use crate::parameter::Parameter;
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

        let parameter = Parameter::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();

        assert_eq!("Name", parameter.name.unwrap());
        assert_eq!("Value", parameter.value.unwrap());
    }
}

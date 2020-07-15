
use quick_xml::Reader;


use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;

pub const GEOCODE_TAG: &[u8] = b"geocode";

pub struct Geocode {
    name: String,
    value: String,
}

impl Geocode {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Geocode, DeserialiseError> {
        let (name, value) = parse_name_value_pair(reader, namespace, GEOCODE_TAG, buf, ns_buf)?;

        return Ok(Geocode { name: name, value: value });
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::{VERSION_1_2};
    use crate::common_alerting_protocol::geocode::Geocode;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<geocode xmlns="urn:oasis:names:tc:emergency:cap:1.2">
                         <valueName>Name</valueName>
                         <value>Value</value>
                     </geocode>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);
        let geocode = Geocode::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();
        assert_eq!("Name", geocode.name);
        assert_eq!("Value", geocode.value);
    }
}

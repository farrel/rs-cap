use crate::result::Result;
use crate::utilities::parse_name_value_pair;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

pub const GEOCODE_TAG: &[u8] = b"geocode";

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Geocode {
    name: Option<String>,
    value: Option<String>,
}

impl Geocode {
    pub fn initialise() -> Geocode {
        Geocode { name: None, value: None }
    }

    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Option<Geocode>> {
        match parse_name_value_pair(reader, namespace, GEOCODE_TAG, buf, ns_buf)? {
            (None, None) => Ok(None),
            (name, value) => Ok(Some(Geocode { name, value })),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alert::VERSION_1_2;
    use crate::geocode::Geocode;
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
        reader.read_namespaced_event(&mut buf, &mut ns_buf).unwrap();
        let geocode = Geocode::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf)
            .unwrap()
            .unwrap();
        assert_eq!("Name", geocode.name.unwrap());
        assert_eq!("Value", geocode.value.unwrap());

        let xml = r#"<geocode xmlns="urn:oasis:names:tc:emergency:cap:1.2">
                     </geocode>"#;
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf).unwrap();
        let geocode = Geocode::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();
        assert_eq!(None, geocode);
    }
}

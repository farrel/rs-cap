use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;

const RESOURCE_TAG: &str = "resource";
const RESOURCE_DESC_TAG: &str = "resourceDesc";
const MIME_TYPE_TAG: &str = "mimeType";
const SIZE_TAG: &str = "size";
const URI_TAG: &str = "uri";
const DEREF_URI_TAG: &str = "derefUri";
const DIGEST_TAG: &str = "digest";

pub struct Resource {
    resource_desc: String,
    mime_type: Option<String>,
    size: Option<u64>,
    uri: Option<String>,
    deref_uri: Option<String>,
    digest: Option<String>,
}

impl Resource {
    fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Resource, DeserialiseError> {
        let mut resource = Resource {
            resource_desc: String::new(),
            mime_type: None,
            size: None,
            uri: None,
            deref_uri: None,
            digest: None,
        };

        let vec = &mut Vec::new();

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(ref e)) if ns == namespace => match str::from_utf8(e.name())? {
                    RESOURCE_DESC_TAG => resource.resource_desc = reader.read_text(RESOURCE_DESC_TAG, vec)?,
                    MIME_TYPE_TAG => resource.mime_type = Some(reader.read_text(MIME_TYPE_TAG, vec)?),
                    SIZE_TAG => resource.size = Some(reader.read_text(SIZE_TAG, vec)?.parse::<u64>()?),
                    URI_TAG => resource.uri = Some(reader.read_text(URI_TAG, vec)?),
                    DEREF_URI_TAG => resource.deref_uri = Some(reader.read_text(DEREF_URI_TAG, vec)?),
                    DIGEST_TAG => resource.digest = Some(reader.read_text(DIGEST_TAG, vec)?),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },

                (Some(ns), Event::End(ref e)) if ns == namespace => match str::from_utf8(e.name())? {
                    RESOURCE_TAG => return Ok(resource),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                (_ns, event) => return Err(DeserialiseError::unknown_event(event)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
    use crate::common_alerting_protocol::resource::Resource;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<resource  xmlns="urn:oasis:names:tc:emergency:cap:1.2">
                       <resourceDesc>map</resourceDesc>
                       <size>100</size>
                       <mimeType>text/html</mimeType>
                       <uri>http://www.rfs.nsw.gov.au/dsp_content.cfm?CAT_ID=683</uri>
                     </resource>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);

        let resource = Resource::deserialize_from_xml(VERSION_1_2, reader, &mut buf, &mut ns_buf).unwrap();
        assert_eq!("map", resource.resource_desc);
        assert_eq!(100, resource.size.unwrap());
        assert_eq!("text/html", resource.mime_type.unwrap());
        assert_eq!("http://www.rfs.nsw.gov.au/dsp_content.cfm?CAT_ID=683", resource.uri.unwrap());
    }
}

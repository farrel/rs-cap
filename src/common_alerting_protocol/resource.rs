use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::str;

const RESOURCE_TAG: &[u8] = b"resource";
const RESOURCE_DESC_TAG: &[u8] = b"resourceDesc";
const MIME_TYPE_TAG: &[u8] = b"mimeType";
const SIZE_TAG: &[u8] = b"size";
const URI_TAG: &[u8] = b"uri";
const DEREF_URI_TAG: &[u8] = b"derefUri";
const DIGEST_TAG: &[u8] = b"digest";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    pub resource_desc: Option<String>,
    mime_type: Option<String>,
    pub size: Option<u64>,
    uri: Option<String>,
    deref_uri: Option<String>,
    digest: Option<String>,
}

impl Resource {
    pub fn initialise() -> Resource {
        Resource {
            resource_desc: None,
            mime_type: None,
            size: None,
            uri: None,
            deref_uri: None,
            digest: None,
        }
    }
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> DeserialiseResult<Resource> {
        let mut resource = Resource::initialise();

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(ref e)) if ns == namespace => match e.local_name() {
                    RESOURCE_DESC_TAG => resource.resource_desc = Some(read_string(namespace, reader, buf, ns_buf, RESOURCE_DESC_TAG)?),
                    MIME_TYPE_TAG => resource.mime_type = Some(read_string(namespace, reader, buf, ns_buf, MIME_TYPE_TAG)?),
                    SIZE_TAG => resource.size = Some(read_string(namespace, reader, buf, ns_buf, SIZE_TAG)?.parse::<u64>()?),
                    URI_TAG => resource.uri = Some(read_string(namespace, reader, buf, ns_buf, URI_TAG)?),
                    DEREF_URI_TAG => resource.deref_uri = Some(read_string(namespace, reader, buf, ns_buf, DEREF_URI_TAG)?),
                    DIGEST_TAG => resource.digest = Some(read_string(namespace, reader, buf, ns_buf, DIGEST_TAG)?),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(&str::from_utf8(unknown_tag)?)),
                },

                (Some(ns), Event::End(ref e)) if ns == namespace => match e.local_name() {
                    RESOURCE_TAG => return Ok(resource),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(&str::from_utf8(unknown_tag)?)),
                },
                (_ns, event) => return Err(DeserialiseError::unknown_event(event)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::VERSION_1_2;
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

        let resource = Resource::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();
        assert_eq!("map", resource.resource_desc.unwrap());
        assert_eq!(100, resource.size.unwrap());
        assert_eq!("text/html", resource.mime_type.unwrap());
        assert_eq!("http://www.rfs.nsw.gov.au/dsp_content.cfm?CAT_ID=683", resource.uri.unwrap());
    }
}

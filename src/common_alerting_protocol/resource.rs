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
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Resource, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        let mut resource = Resource {
            resource_desc: String::new(),
            mime_type: None,
            size: None,
            uri: None,
            deref_uri: None,
            digest: None,
        };

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (ref _ns, Event::Start(ref e)) => match str::from_utf8(e.name())? {
                    RESOURCE_TAG => (),
                    RESOURCE_DESC_TAG => resource.resource_desc = parse_string(reader, RESOURCE_DESC_TAG)?,
                    MIME_TYPE_TAG => resource.mime_type = Some(parse_string(reader, MIME_TYPE_TAG)?),
                    SIZE_TAG => resource.size = parse_u64(reader, SIZE_TAG)?,
                    URI_TAG => resource.uri = Some(parse_string(reader, URI_TAG)?),
                    DEREF_URI_TAG => resource.deref_uri = Some(parse_string(reader, DEREF_URI_TAG)?),
                    DIGEST_TAG => resource.digest = Some(parse_string(reader, DIGEST_TAG)?),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },

                (ref _ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                    RESOURCE_TAG => return Ok(resource),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::resource::Resource;
    use quick_xml::Reader;

    #[test]
    fn test_deserialize_from_xml() {
        let xml = r#"<resource>
                      <resourceDesc>map</resourceDesc>
                      <mimeType>text/html</mimeType>
                      <uri>http://www.rfs.nsw.gov.au/dsp_content.cfm?CAT_ID=683</uri>
                    </resource>"#;

        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);

        let resource = Resource::deserialize_from_xml(reader).unwrap();
        assert_eq!("map", resource.resource_desc);
        assert_eq!("text/html", resource.mime_type.unwrap());
        assert_eq!("http://www.rfs.nsw.gov.au/dsp_content.cfm?CAT_ID=683", resource.uri.unwrap());
    }
}

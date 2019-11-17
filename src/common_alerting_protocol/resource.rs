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
    mime_type: String,
    size: u64,
    uri: String,
    deref_uri: String,
    digest: String,
}

impl Resource {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Resource, DeserialiseError> {
        let mut text = String::new();
        let mut resource_desc = String::new();
        let mut mime_type = String::new();
        let mut size: u64 = 0;
        let mut uri = String::new();
        let mut deref_uri = String::new();
        let mut digest = String::new();

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (ref _ns, Event::Start(ref e)) => match str::from_utf8(e.name())? {
                    RESOURCE_TAG | RESOURCE_DESC_TAG | MIME_TYPE_TAG | SIZE_TAG | URI_TAG | DEREF_URI_TAG | DIGEST_TAG => (),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },

                (_ns, Event::Text(e)) => text.push_str(&e.unescape_and_decode(reader)?),

                (ref _ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                    RESOURCE_TAG => {
                        return Ok(Resource {
                            resource_desc: resource_desc,
                            mime_type: mime_type,
                            size: size,
                            uri: uri,
                            deref_uri: deref_uri,
                            digest: digest,
                        })
                    }
                    RESOURCE_DESC_TAG => {
                        resource_desc.push_str(&text);
                        text.clear()
                    }
                    MIME_TYPE_TAG => {
                        mime_type.push_str(&text);
                        text.clear()
                    }
                    SIZE_TAG => {
                        size = text.parse::<u64>()?;
                        text.clear()
                    }
                    URI_TAG => {
                        uri.push_str(&text);
                        text.clear()
                    }
                    DEREF_URI_TAG => {
                        deref_uri.push_str(&text);
                        text.clear()
                    }
                    DIGEST_TAG => {
                        digest.push_str(&text);
                        text.clear()
                    }
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
        assert_eq!("text/html", resource.mime_type);
        assert_eq!("http://www.rfs.nsw.gov.au/dsp_content.cfm?CAT_ID=683", resource.uri);
    }
}

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use chrono::prelude::*;
use chrono::DateTime;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;
use std::str::FromStr;

const NAME_TAG: &[u8] = b"valueName";
const VALUE_TAG: &[u8] = b"value";

pub fn parse_name_value_pair(
    reader: &mut Reader<&[u8]>,
    namespace: &[u8],
    end_tag: &[u8],
    buf: &mut std::vec::Vec<u8>,
    ns_buf: &mut std::vec::Vec<u8>,
) -> Result<(String, String), DeserialiseError> {
    let mut name = String::new();
    let mut value = String::new();

    loop {
        buf.clear();
        match reader.read_namespaced_event(buf, ns_buf)? {
            (_, Event::Eof) => return Err(DeserialiseError::error("Unexpected EOF")),
            (Some(ref ns), Event::Start(ref e)) => match e.local_name() {
                NAME_TAG if *ns == namespace => {
                    name.push_str(reader.read_text(NAME_TAG, &mut Vec::new())?.as_str());
                }
                VALUE_TAG if *ns == namespace => {
                    value.push_str(reader.read_text(VALUE_TAG, &mut Vec::new())?.as_str());
                }
                unknown_tag => return Err(DeserialiseError::tag_not_recognised(str::from_utf8(unknown_tag)?)),
            },
            (Some(ref ns), Event::End(ref e)) => {
                if *ns == namespace {
                    if e.local_name() == end_tag {
                        return Ok((name, value));
                    }
                } else {
                    return Err(DeserialiseError::tag_not_expected(str::from_utf8(e.local_name())?));
                }
            }
            (Some(namespace), event) => (),
            (None, event) => return Err(DeserialiseError::unknown_event(event)),
        }
    }
}

#[cfg(test)]
mod tests {
    //use crate::common_alerting_protocol::utilities::parse_end_tag;
    //use crate::common_alerting_protocol::utilities::parse_start_tag;
    //use crate::common_alerting_protocol::utilities::parse_string;
    //use crate::common_alerting_protocol::utilities::parse_tags;
    //use quick_xml::Reader;

    //#[test]
    //fn test_parse_tags() {
    //    let tag = b"eventCode";
    //    let namespace = b"a";

    //    let xml = r#"<eventCode xmlns="a">EVENT CODE</eventCode>"#;
    //    let reader = &mut Reader::from_str(xml);
    //    reader.trim_text(true);

    //    parse_tags(reader, tag, namespace);
    //}
}

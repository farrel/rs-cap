use crate::common_alerting_protocol::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
use crate::common_alerting_protocol::deserialise_error::DeserialiseError;

use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

pub type DeserialiseResult<T> = std::result::Result<T, DeserialiseError>;

const NAME_TAG: &[u8] = b"valueName";
const VALUE_TAG: &[u8] = b"value";

pub fn look_for_cap_namespace(xml_string: &str) -> Option<&str> {
    if xml_string.find(VERSION_1_0).is_some() {
        Some(VERSION_1_0)
    } else if xml_string.find(VERSION_1_1).is_some() {
        Some(VERSION_1_1)
    } else if xml_string.find(VERSION_1_2).is_some() {
        Some(VERSION_1_2)
    } else {
        None
    }
}

pub fn parse_name_value_pair(
    reader: &mut Reader<&[u8]>,
    namespace: &[u8],
    end_tag: &[u8],
    buf: &mut std::vec::Vec<u8>,
    ns_buf: &mut std::vec::Vec<u8>,
) -> DeserialiseResult<(String, String)> {
    let mut name = String::new();
    let mut value = String::new();

    loop {
        match reader.read_namespaced_event(buf, ns_buf)? {
            (_, Event::Eof) => return Err(DeserialiseError::EofReached),
            (Some(ns), Event::Start(e)) => match e.local_name() {
                NAME_TAG if ns == namespace => {
                    name.push_str(read_string(namespace, reader, buf, ns_buf, NAME_TAG)?.as_str());
                }
                VALUE_TAG if ns == namespace => {
                    value.push_str(read_string(namespace, reader, buf, ns_buf, VALUE_TAG)?.as_str());
                }
                unknown_tag => return Err(DeserialiseError::tag_not_recognised(str::from_utf8(unknown_tag)?)),
            },
            (Some(ns), Event::End(e)) => {
                if ns == namespace {
                    if e.local_name() == end_tag {
                        return Ok((name, value));
                    }
                } else {
                    return Err(DeserialiseError::tag_not_expected(str::from_utf8(e.local_name())?));
                }
            }
            (Some(_namespace), _event) => (),
            (None, event) => return Err(DeserialiseError::unknown_event(event)),
        }
    }
}

pub fn read_string(
    namespace: &[u8],
    reader: &mut Reader<&[u8]>,
    buf: &mut std::vec::Vec<u8>,
    ns_buf: &mut std::vec::Vec<u8>,
    closing_tag: &[u8],
) -> DeserialiseResult<String> {
    let mut string = String::new();

    loop {
        match reader.read_namespaced_event(buf, ns_buf)? {
            (None, Event::Text(text)) => string.push_str(&text.unescape_and_decode(&reader)?),
            (Some(ns), Event::End(end)) if ns == namespace && end.local_name() == closing_tag => return Ok(string),
            _ => return Err(DeserialiseError::error(&format!("No end tag found: {}", str::from_utf8(closing_tag)?))),
        }
    }
}

pub fn split_string(string: &str) -> DeserialiseResult<Vec<&str>> {
    return Ok(string.split(' ').collect());
}

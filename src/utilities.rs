use crate::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
use crate::error::Error;
use crate::result::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

const NAME_TAG: &[u8] = b"valueName";
const VALUE_TAG: &[u8] = b"value";

pub fn look_for_cap_namespace(xml_string: &str) -> Option<&str> {
    if xml_string.contains(VERSION_1_0) {
        Some(VERSION_1_0)
    } else if xml_string.contains(VERSION_1_1) {
        Some(VERSION_1_1)
    } else if xml_string.contains(VERSION_1_2) {
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
) -> Result<(Option<String>, Option<String>)> {
    let mut name: Option<String> = None;
    let mut value: Option<String> = None;

    loop {
        match reader.read_namespaced_event(buf, ns_buf)? {
            (_, Event::Eof) => return Err(Error::EofReached),
            (Some(ns), Event::Start(e)) => match e.local_name() {
                NAME_TAG if ns == namespace => name = read_string(namespace, reader, buf, ns_buf, NAME_TAG)?,
                VALUE_TAG if ns == namespace => value = read_string(namespace, reader, buf, ns_buf, VALUE_TAG)?,
                unknown_tag => return Err(Error::tag_not_recognised(str::from_utf8(unknown_tag)?)),
            },
            (Some(ns), Event::End(e)) => {
                if ns == namespace {
                    if e.local_name() == end_tag {
                        return Ok((name, value));
                    }
                } else {
                    return Err(Error::tag_not_expected(str::from_utf8(e.local_name())?));
                }
            }
            (Some(_namespace), _event) => (),
            (None, event) => return Err(Error::unknown_event(event)),
        }
    }
}

pub fn read_string(
    namespace: &[u8],
    reader: &mut Reader<&[u8]>,
    buf: &mut std::vec::Vec<u8>,
    ns_buf: &mut std::vec::Vec<u8>,
    closing_tag: &[u8],
) -> Result<Option<String>> {
    let mut string = String::new();

    loop {
        match reader.read_namespaced_event(buf, ns_buf)? {
            (None, Event::Text(text)) => string.push_str(&text.unescape_and_decode(reader)?),
            (Some(ns), Event::End(end)) if ns == namespace && end.local_name() == closing_tag => {
                if !string.is_empty() {
                    return Ok(Some(string));
                } else {
                    return Ok(None);
                }
            }
            _ => return Err(Error::Other(format!("No end tag found: {}", str::from_utf8(closing_tag)?))),
        }
    }
}

pub fn split_string(string: &str) -> Result<Vec<&str>> {
    return Ok(string.split(' ').collect());
}

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use chrono::prelude::*;
use chrono::DateTime;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;
use std::str::FromStr;

pub fn parse_string(reader: &mut Reader<&[u8]>, tag: &str) -> Result<String, DeserialiseError> {
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let mut string = String::new();

    loop {
        match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
            (_ns, Event::Start(ref e)) => return Err(DeserialiseError::tag_not_expected(str::from_utf8(e.name())?)),
            (_ns, Event::Text(e)) => string.push_str(&e.unescape_and_decode(reader)?),
            (_ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                tag => return Ok(string),
                unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
            },
            _ => (),
        }
    }
}

pub fn parse_f64(reader: &mut Reader<&[u8]>, tag: &str) -> Result<Option<f64>, DeserialiseError> {
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let mut f64_string = String::new();

    loop {
        match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
            (_ns, Event::Start(ref e)) => return Err(DeserialiseError::tag_not_expected(str::from_utf8(e.name())?)),
            (_ns, Event::Text(e)) => f64_string.push_str(&e.unescape_and_decode(reader)?),
            (_ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                tag => return Ok(Some(f64_string.parse::<f64>()?)),
                unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
            },
            _ => (),
        }
    }
}

pub fn parse_u64(reader: &mut Reader<&[u8]>, tag: &str) -> Result<Option<u64>, DeserialiseError> {
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let mut u64_string = String::new();

    loop {
        match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
            (_ns, Event::Start(ref e)) => return Err(DeserialiseError::tag_not_expected(str::from_utf8(e.name())?)),
            (_ns, Event::Text(e)) => u64_string.push_str(&e.unescape_and_decode(reader)?),
            (_ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                tag => return Ok(Some(u64_string.parse::<u64>()?)),
                unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
            },
            _ => (),
        }
    }
}

pub fn parse_datetime(reader: &mut Reader<&[u8]>, tag: &str) -> Result<DateTime<FixedOffset>, DeserialiseError> {
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let mut date_string = String::new();

    loop {
        match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
            (_ns, Event::Start(ref e)) => return Err(DeserialiseError::tag_not_expected(str::from_utf8(e.name())?)),
            (_ns, Event::Text(e)) => date_string.push_str(&e.unescape_and_decode(reader)?),
            (_ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                tag => {
                    if let Ok(parsed_datetime) = DateTime::parse_from_rfc3339(&date_string) {
                        return Ok(parsed_datetime);
                    } else {
                        return Err(DeserialiseError::enum_not_found(&date_string));
                    }
                }
                unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
            },
            _ => (),
        }
    }
}

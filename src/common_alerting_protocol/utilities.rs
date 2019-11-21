use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;

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

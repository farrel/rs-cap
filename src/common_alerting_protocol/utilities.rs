use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use chrono::prelude::*;
use chrono::DateTime;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;
use std::str::FromStr;

//pub fn parse_start_tag<'a>(reader: &'a mut Reader<&[u8]>, namespace: &[u8]) -> Result<String, DeserialiseError> {
//    let mut buf = Vec::new();
//    let mut ns_buf = Vec::new();
//    let mut start_tag = String::new();
//
//    match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
//        (ref ns, Event::Start(ref e)) => match (*ns, e.local_name()) {
//            (Some(namespace), tag) => {
//                start_tag.push_str(str::from_utf8(tag)?);
//                return Ok(start_tag);
//            }
//            (_ns, unknown_tag) => return Err(DeserialiseError::tag_not_expected(str::from_utf8(unknown_tag)?)),
//        },
//        _ => return Err(DeserialiseError::error("No start tag found")),
//    }
//}
//
//pub fn parse_end_tag(reader: &mut Reader<&[u8]>, tag: &[u8], namespace: &[u8]) -> Result<(), DeserialiseError> {
//    let mut buf = Vec::new();
//    let mut end_tag = String::new();
//
//    match reader.read_event(&mut buf)? {
//        Event::End(ref e) => match e.local_name() {
//            tag => {
//                return Ok(());
//            } //    unknown_tag => {
//              //        return Err(DeserialiseError::tag_not_expected(str::from_utf8(unknown_tag)?));
//              //    }
//        },
//        _ => {
//            return Err(DeserialiseError::error("No end tag found"));
//        }
//    }
//}
//
//pub fn parse_string(reader: &mut Reader<&[u8]>) -> Result<String, DeserialiseError> {
//    let mut buf = Vec::new();
//    let mut ns_buf = Vec::new();
//    let mut string = String::new();
//
//    match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
//        (None, Event::Text(e)) => {
//            let unstring = e.unescape_and_decode(&reader)?;
//            string.push_str(&unstring);
//        }
//        _ => return Err(DeserialiseError::error("No text found")),
//    }
//
//    return Ok(string);
//}
//
//pub fn parse_f64(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<Option<f64>, DeserialiseError> {
//    let mut buf = Vec::new();
//    let mut ns_buf = Vec::new();
//    let mut f64_string = String::new();
//
//    loop {
//        match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
//            (_ns, Event::Start(ref e)) => return Err(DeserialiseError::tag_not_expected(str::from_utf8(e.name())?)),
//            (_ns, Event::Text(e)) => f64_string.push_str(&e.unescape_and_decode(reader)?),
//            (_ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
//                tag => return Ok(Some(f64_string.parse::<f64>()?)),
//                unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
//            },
//            _ => (),
//        }
//    }
//}
//
//pub fn parse_u64(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<Option<u64>, DeserialiseError> {
//    let mut buf = Vec::new();
//    let mut ns_buf = Vec::new();
//    let mut u64_string = String::new();
//
//    loop {
//        match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
//            (_ns, Event::Start(ref e)) => return Err(DeserialiseError::tag_not_expected(str::from_utf8(e.name())?)),
//            (_ns, Event::Text(e)) => u64_string.push_str(&e.unescape_and_decode(reader)?),
//            (_ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
//                tag => return Ok(Some(u64_string.parse::<u64>()?)),
//                unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
//            },
//            _ => (),
//        }
//    }
//}

pub fn parse_datetime(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<DateTime<FixedOffset>, DeserialiseError> {
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

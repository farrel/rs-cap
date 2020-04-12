//use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
//use crate::common_alerting_protocol::info::{Info, INFO_TAG};
//use crate::common_alerting_protocol::utilities::*;
//use chrono::prelude::*;
//use quick_xml::events::Event;
//use quick_xml::Reader;
//use std::str;

#[derive(Debug)]
pub enum Version {
    V1_0,
    V1_1,
    V1_2,
}

pub const VERSION_1_0: &[u8] = b"urn:oasis:names:tc:emergency:cap:1.0";
pub const VERSION_1_1: &[u8] = b"urn:oasis:names:tc:emergency:cap:1.1";
pub const VERSION_1_2: &[u8] = b"urn:oasis:names:tc:emergency:cap:1.2";

//const ALERT_TAG: &str = "alert";
//
//pub struct Alert {
//    pub version: Option<Version>,
//    pub identifier: Option<String>,
//    pub sender: Option<String>,
//    pub sent: Option<DateTime<Utc>>,
//    pub status: Option<String>,
//    pub msg_type: Option<String>,
//    pub scope: Option<String>,
//    pub source: Option<String>,
//    pub restriction: Option<String>,
//    pub notes: Option<String>,
//    pub addresses: Vec<String>,
//    pub codes: Vec<String>,
//    pub references: Vec<String>,
//    pub incidents: Vec<String>,
//    pub infos: Vec<Info>,
//}
//
//impl Alert {
//    pub fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Alert, DeserialiseError> {
//        let mut buf = Vec::new();
//        let mut ns_buf = Vec::new();
//
//        let mut alert = Alert {
//            version: None,
//            identifier: None,
//            sender: None,
//            sent: None,
//            status: None,
//            msg_type: None,
//            scope: None,
//            source: None,
//            restriction: None,
//            notes: None,
//            addresses: Vec::new(),
//            codes: Vec::new(),
//            references: Vec::new(),
//            incidents: Vec::new(),
//            infos: Vec::new(),
//        };
//
//        loop {
//            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
//                (Some(namespace), Event::Start(ref e)) => match str::from_utf8(e.name())? {
//                    ALERT_TAG => (),
//                    INFO_TAG => alert.infos.push(Info::deserialize_from_xml(namespace, reader)?),
//                    _ => (),
//                },
//                _ => (),
//            }
//        }
//    }
//}

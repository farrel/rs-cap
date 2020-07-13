use crate::common_alerting_protocol::deserialise_error::{DeserialiseError, ParseEnumError};
use crate::common_alerting_protocol::info::{Certainty, Info, INFO_TAG};
use crate::common_alerting_protocol::utilities::*;
use chrono::prelude::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fmt;
use std::str;
use std::str::FromStr;

pub const VERSION_1_0: &str = "urn:oasis:names:tc:emergency:cap:1.0";
pub const VERSION_1_1: &str = "urn:oasis:names:tc:emergency:cap:1.1";
pub const VERSION_1_2: &str = "urn:oasis:names:tc:emergency:cap:1.2";

const ALERT_TAG: &[u8] = b"alert";
const IDENTIFIER_TAG: &[u8] = b"identifier";
const INCIDENTS_TAG: &[u8] = b"incidents";
const SENDER_TAG: &[u8] = b"sender";
const SENT_TAG: &[u8] = b"sent";
const STATUS_TAG: &[u8] = b"status";
const MSG_TYPE_TAG: &[u8] = b"msgType";
const SOURCE_TAG: &[u8] = b"source";
const SCOPE_TAG: &[u8] = b"scope";
const CODE_TAG: &[u8] = b"code";
const NOTE_TAG: &[u8] = b"note";
const REFERENCES_TAG: &[u8] = b"references";
const RESTRICTION_TAG: &[u8] = b"restriction";

#[derive(Debug)]
pub enum Version {
    V1_0,
    V1_1,
    V1_2,
}

#[derive(PartialEq, fmt::Debug)]
pub enum Status {
    Actual,
    Exercise,
    System,
    Test,
    Draft,
}

impl FromStr for Status {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> Result<Status, ParseEnumError> {
        match enum_string {
            "Actual" => Ok(Status::Actual),
            "Exercise" => Ok(Status::Exercise),
            "System" => Ok(Status::System),
            "Test" => Ok(Status::Test),
            "Draft" => Ok(Status::Draft),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

#[derive(fmt::Debug, PartialEq)]
pub enum MsgType {
    Alert,
    Update,
    Cancel,
    Ack,
    Error,
}

impl FromStr for MsgType {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> Result<MsgType, ParseEnumError> {
        match enum_string {
            "Alert" => Ok(MsgType::Alert),
            "Update" => Ok(MsgType::Update),
            "Cancel" => Ok(MsgType::Cancel),
            "Ack" => Ok(MsgType::Ack),
            "Error" => Ok(MsgType::Error),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

#[derive(fmt::Debug, PartialEq)]
pub enum Scope {
    Public,
    Restricted,
    Private,
}

impl FromStr for Scope {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> Result<Scope, ParseEnumError> {
        match enum_string {
            "Public" => Ok(Scope::Public),
            "Restricted" => Ok(Scope::Restricted),
            "Private" => Ok(Scope::Private),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

pub struct Alert {
    pub version: Option<Version>,
    pub identifier: Option<String>,
    pub sender: Option<String>,
    pub sent: Option<DateTime<FixedOffset>>,
    pub status: Option<Status>,
    pub msg_type: Option<MsgType>,
    pub scope: Option<Scope>,
    pub source: Option<String>,
    pub restriction: Option<String>,
    pub note: Option<String>,
    pub addresses: Vec<String>,
    pub codes: Vec<String>,
    pub references: Vec<String>,
    pub incidents: Option<String>,
    pub infos: Vec<Info>,
}

impl Alert {
    pub fn initialise() -> Alert {
        return Alert {
            version: None,
            identifier: None,
            sender: None,
            sent: None,
            status: None,
            msg_type: None,
            scope: None,
            source: None,
            restriction: None,
            note: None,
            addresses: Vec::new(),
            codes: Vec::new(),
            references: Vec::new(),
            incidents: None,
            infos: Vec::new(),
        };
    }
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Alert, DeserialiseError> {
        let mut alert = Alert::initialise();

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(e)) if ns == namespace => match e.local_name() {
                    ALERT_TAG => (),

                    CODE_TAG => alert.codes.push(read_string(namespace, reader, buf, ns_buf, CODE_TAG)?),
                    IDENTIFIER_TAG => alert.identifier = Some(read_string(namespace, reader, buf, ns_buf, IDENTIFIER_TAG)?),
                    INCIDENTS_TAG => alert.incidents = Some(read_string(namespace, reader, buf, ns_buf, INCIDENTS_TAG)?),
                    MSG_TYPE_TAG => alert.msg_type = Some(read_string(namespace, reader, buf, ns_buf, MSG_TYPE_TAG)?.parse::<MsgType>()?),
                    NOTE_TAG => alert.note = Some(read_string(namespace, reader, buf, ns_buf, NOTE_TAG)?),
                    REFERENCES_TAG => split_string(&read_string(namespace, reader, buf, ns_buf, REFERENCES_TAG)?)?
                        .into_iter()
                        .for_each(|reference| alert.references.push(reference.to_string())),
                    RESTRICTION_TAG => alert.restriction = Some(read_string(namespace, reader, buf, ns_buf, RESTRICTION_TAG)?),
                    SCOPE_TAG => alert.scope = Some(read_string(namespace, reader, buf, ns_buf, SCOPE_TAG)?.parse::<Scope>()?),
                    SENDER_TAG => alert.sender = Some(read_string(namespace, reader, buf, ns_buf, SENDER_TAG)?),
                    SENT_TAG => alert.sent = Some(DateTime::parse_from_rfc3339(&read_string(namespace, reader, buf, ns_buf, SENT_TAG)?)?),
                    SOURCE_TAG => alert.source = Some(read_string(namespace, reader, buf, ns_buf, SOURCE_TAG)?),
                    STATUS_TAG => alert.status = Some(read_string(namespace, reader, buf, ns_buf, STATUS_TAG)?.parse::<Status>()?),

                    INFO_TAG => alert.infos.push(Info::deserialize_from_xml(namespace, reader, buf, ns_buf)?),

                    unknown_tag => return Err(DeserialiseError::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },

                (Some(ns), Event::End(ref e)) if ns == namespace => match e.local_name() {
                    ALERT_TAG => return Ok(alert),
                    unknown_tag => return Err(DeserialiseError::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },

                (_ns, Event::Eof) => {
                    return Err(DeserialiseError::EofReached);
                }
                _ => (),
            }
        }
    }

    pub fn add_info<F>(&mut self, mut block: F) -> &Info
    where
        F: Fn(&mut Info),
    {
        let mut info = Info::initialise();

        block(&mut info);

        self.infos.push(info);

        return self.infos.last().unwrap();
    }
}

pub fn parse(xml_string: &str) -> Result<Alert, DeserialiseError> {
    let buf = &mut Vec::new();
    let ns_buf = &mut Vec::new();
    let reader = &mut Reader::from_str(xml_string);
    reader.trim_text(true);

    if let Some(namespace) = look_for_cap_namespace(xml_string) {
        Ok(Alert::deserialize_from_xml(namespace.as_bytes(), reader, buf, ns_buf)?)
    } else {
        Err(DeserialiseError::NameSpaceNotFound)
    }
}

#[test]
fn test_add_info() {
    let mut alert = Alert::initialise();

    let info = alert.add_info(|info| {
        info.audience = Some(String::from("Test"));
        info.certainty = Some(Certainty::Observed);
    });

    assert_eq!(Some(String::from("Test")), info.audience);
    assert_eq!(Some(Certainty::Observed), info.certainty);
}

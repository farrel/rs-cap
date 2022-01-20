use crate::common_alerting_protocol::info::{Info, INFO_TAG};
use crate::common_alerting_protocol::reference::Reference;
use crate::common_alerting_protocol::utilities::*;
use crate::common_alerting_protocol::{Error, ParseEnumError, Result};
use chrono::prelude::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str;
use std::str::FromStr;
use uuid::Uuid;

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

const STATUS_ACTUAL: &str = "Actual";
const STATUS_EXERCISE: &str = "Exercise";
const STATUS_SYSTEM: &str = "System";
const STATUS_TEST: &str = "Test";
const STATUS_DRAFT: &str = "Draft";

const MSG_TYPE_ALERT: &str = "Alert";
const MSG_TYPE_UPDATE: &str = "Update";
const MSG_TYPE_CANCEL: &str = "Cancel";
const MSG_TYPE_ACK: &str = "Ack";
const MSG_TYPE_ERROR: &str = "Error";

const SCOPE_PUBLIC: &str = "Public";
const SCOPE_RESTRICTED: &str = "Restricted";
const SCOPE_PRIVATE: &str = "Private";

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Version {
    V1_0,
    V1_1,
    V1_2,
}

impl FromStr for Version {
    type Err = ParseEnumError;

    fn from_str(version_string: &str) -> std::result::Result<Version, ParseEnumError> {
        match version_string {
            VERSION_1_0 => Ok(Version::V1_0),
            VERSION_1_1 => Ok(Version::V1_1),
            VERSION_1_2 => Ok(Version::V1_2),
            _ => Err(ParseEnumError::enum_not_found(version_string)),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Actual,
    Exercise,
    System,
    Test,
    Draft,
}

impl FromStr for Status {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> std::result::Result<Status, ParseEnumError> {
        match enum_string {
            STATUS_ACTUAL => Ok(Status::Actual),
            STATUS_EXERCISE => Ok(Status::Exercise),
            STATUS_SYSTEM => Ok(Status::System),
            STATUS_TEST => Ok(Status::Test),
            STATUS_DRAFT => Ok(Status::Draft),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Status::Actual => write!(f, "{}", STATUS_ACTUAL),
            Status::Exercise => write!(f, "{}", STATUS_EXERCISE),
            Status::System => write!(f, "{}", STATUS_SYSTEM),
            Status::Test => write!(f, "{}", STATUS_TEST),
            Status::Draft => write!(f, "{}", STATUS_DRAFT),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum MsgType {
    Alert,
    Update,
    Cancel,
    Ack,
    Error,
}

impl FromStr for MsgType {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> std::result::Result<MsgType, ParseEnumError> {
        match enum_string {
            MSG_TYPE_ALERT => Ok(MsgType::Alert),
            MSG_TYPE_UPDATE => Ok(MsgType::Update),
            MSG_TYPE_CANCEL => Ok(MsgType::Cancel),
            MSG_TYPE_ACK => Ok(MsgType::Ack),
            MSG_TYPE_ERROR => Ok(MsgType::Error),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

impl Display for MsgType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            MsgType::Alert => write!(f, "{}", MSG_TYPE_ALERT),
            MsgType::Update => write!(f, "{}", MSG_TYPE_UPDATE),
            MsgType::Cancel => write!(f, "{}", MSG_TYPE_CANCEL),
            MsgType::Ack => write!(f, "{}", MSG_TYPE_ACK),
            MsgType::Error => write!(f, "{}", MSG_TYPE_ERROR),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum Scope {
    Public,
    Restricted,
    Private,
}

impl FromStr for Scope {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> std::result::Result<Scope, ParseEnumError> {
        match enum_string {
            SCOPE_PUBLIC => Ok(Scope::Public),
            SCOPE_RESTRICTED => Ok(Scope::Restricted),
            SCOPE_PRIVATE => Ok(Scope::Private),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Scope::Public => write!(f, "{}", SCOPE_PUBLIC),
            Scope::Restricted => write!(f, "{}", SCOPE_RESTRICTED),
            Scope::Private => write!(f, "{}", SCOPE_PRIVATE),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub references: Vec<Reference>,
    pub incidents: Option<String>,
    pub infos: Vec<Info>,
}

impl Alert {
    pub fn initialise() -> Alert {
        Alert {
            version: None,
            identifier: Some(Uuid::new_v4().to_hyphenated().to_string()),
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
        }
    }

    pub fn deserialize_from_xml(namespace: &[u8], reader: &mut Reader<&[u8]>, buf: &mut std::vec::Vec<u8>, ns_buf: &mut std::vec::Vec<u8>) -> Result<Alert> {
        let mut alert = Alert::initialise();

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(e)) if ns == namespace => match e.local_name() {
                    ALERT_TAG => alert.version = Some(str::from_utf8(namespace)?.parse::<Version>()?),

                    CODE_TAG => {
                        if let Some(string) = read_string(namespace, reader, buf, ns_buf, CODE_TAG)? {
                            alert.codes.push(string)
                        }
                    }
                    IDENTIFIER_TAG => alert.identifier = read_string(namespace, reader, buf, ns_buf, IDENTIFIER_TAG)?,
                    INCIDENTS_TAG => alert.incidents = read_string(namespace, reader, buf, ns_buf, INCIDENTS_TAG)?,
                    MSG_TYPE_TAG => {
                        alert.msg_type = read_string(namespace, reader, buf, ns_buf, MSG_TYPE_TAG)?.and_then(|string| string.parse::<MsgType>().ok())
                    }

                    NOTE_TAG => alert.note = read_string(namespace, reader, buf, ns_buf, NOTE_TAG)?,
                    REFERENCES_TAG => {
                        if let Some(string) = read_string(namespace, reader, buf, ns_buf, REFERENCES_TAG)? {
                            for reference_str in split_string(&string)? {
                                alert.references.push(Reference::parse_string(reference_str)?)
                            }
                        }
                    }
                    RESTRICTION_TAG => alert.restriction = read_string(namespace, reader, buf, ns_buf, RESTRICTION_TAG)?,
                    SCOPE_TAG => alert.scope = read_string(namespace, reader, buf, ns_buf, SCOPE_TAG)?.and_then(|string| string.parse::<Scope>().ok()),
                    SENDER_TAG => alert.sender = read_string(namespace, reader, buf, ns_buf, SENDER_TAG)?,
                    SENT_TAG => {
                        alert.sent = read_string(namespace, reader, buf, ns_buf, SENT_TAG)?.and_then(|string| DateTime::parse_from_rfc3339(&string).ok())
                    }
                    SOURCE_TAG => alert.source = read_string(namespace, reader, buf, ns_buf, SOURCE_TAG)?,
                    STATUS_TAG => alert.status = read_string(namespace, reader, buf, ns_buf, STATUS_TAG)?.and_then(|string| string.parse::<Status>().ok()),

                    INFO_TAG => alert.infos.push(Info::deserialize_from_xml(namespace, reader, buf, ns_buf)?),

                    unknown_tag => return Err(Error::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },

                (Some(ns), Event::End(ref e)) if ns == namespace => match e.local_name() {
                    ALERT_TAG => return Ok(alert),
                    unknown_tag => return Err(Error::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },

                (_ns, Event::Eof) => {
                    return Err(Error::EofReached);
                }
                _ => (),
            }
        }
    }

    pub fn add_info<F>(&mut self, build_info: F)
    where
        F: Fn(&mut Info),
    {
        let mut info = Info::initialise();
        build_info(&mut info);
        self.infos.push(info);
    }
}

impl Display for Alert {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.identifier.clone() {
            Some(value) => write!(f, "{}", value),
            None => write!(f, ""),
        }
    }
}

pub fn parse(xml_string: &str) -> Result<Alert> {
    let buf = &mut Vec::new();
    let ns_buf = &mut Vec::new();
    let reader = &mut Reader::from_str(xml_string);
    reader.trim_text(true);

    if let Some(namespace) = look_for_cap_namespace(xml_string) {
        Ok(Alert::deserialize_from_xml(namespace.as_bytes(), reader, buf, ns_buf)?)
    } else {
        Err(Error::NameSpaceNotFound)
    }
}

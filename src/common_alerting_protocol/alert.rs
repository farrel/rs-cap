use crate::common_alerting_protocol::deserialise_error::{DeserialiseError, ParseEnumError};
use crate::common_alerting_protocol::info::{Info, INFO_TAG};
use crate::common_alerting_protocol::utilities::*;
use chrono::prelude::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fmt;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub enum Version {
    V1_0,
    V1_1,
    V1_2,
}

pub const VERSION_1_0: &[u8] = b"urn:oasis:names:tc:emergency:cap:1.0";
pub const VERSION_1_1: &[u8] = b"urn:oasis:names:tc:emergency:cap:1.1";
pub const VERSION_1_2: &[u8] = b"urn:oasis:names:tc:emergency:cap:1.2";

const ALERT_TAG: &[u8] = b"alert";
const IDENTIFIER_TAG: &[u8] = b"identifier";
const SENDER_TAG: &[u8] = b"sender";
const STATUS_TAG: &[u8] = b"status";
const MSG_TYPE_TAG: &[u8] = b"msgType";
const SOURCE_TAG: &[u8] = b"source";
const SCOPE_TAG: &[u8] = b"scope";
const CODE_TAG: &[u8] = b"code";

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
    pub sent: Option<DateTime<Utc>>,
    pub status: Option<Status>,
    pub msg_type: Option<MsgType>,
    pub scope: Option<Scope>,
    pub source: Option<String>,
    pub restriction: Option<String>,
    pub notes: Option<String>,
    pub addresses: Vec<String>,
    pub codes: Vec<String>,
    pub references: Vec<String>,
    pub incidents: Vec<String>,
    pub infos: Vec<Info>,
}

impl Alert {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Alert, DeserialiseError> {
        let mut alert = Alert {
            version: None,
            identifier: None,
            sender: None,
            sent: None,
            status: None,
            msg_type: None,
            scope: None,
            source: None,
            restriction: None,
            notes: None,
            addresses: Vec::new(),
            codes: Vec::new(),
            references: Vec::new(),
            incidents: Vec::new(),
            infos: Vec::new(),
        };

        let mut vec = Vec::new();

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(ref e)) if ns == namespace => match e.local_name() {
                    ALERT_TAG => (),
                    IDENTIFIER_TAG => alert.identifier = Some(reader.read_text(IDENTIFIER_TAG, &mut vec)?),
                    SENDER_TAG => alert.sender = Some(reader.read_text(SENDER_TAG, &mut vec)?),
                    STATUS_TAG => alert.status = Some(reader.read_text(STATUS_TAG, &mut vec)?.parse::<Status>()?),
                    MSG_TYPE_TAG => alert.msg_type = Some(reader.read_text(MSG_TYPE_TAG, &mut vec)?.parse::<MsgType>()?),
                    SOURCE_TAG => alert.source = Some(reader.read_text(SOURCE_TAG, &mut vec)?),
                    SCOPE_TAG => alert.scope = Some(reader.read_text(SCOPE_TAG, &mut vec)?.parse::<Scope>()?),
                    CODE_TAG => alert.codes.push(reader.read_text(CODE_TAG, &mut vec)?),

                    INFO_TAG => alert.infos.push(Info::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    _ => (),
                },

                (Some(ns), Event::End(ref e)) if ns == namespace => match e.local_name() {
                    ALERT_TAG => return Ok(alert),
                    _unknown_tag => (),
                },
                (_ns, Event::Eof) => return Err(DeserialiseError::EofReached),
                _ => (),
            }
        }
    }
}

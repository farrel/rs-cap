use crate::common_alerting_protocol::area::{Area, AREA_TAG};
use crate::common_alerting_protocol::event_code::EventCode;
use crate::common_alerting_protocol::parameter::{Parameter, PARAMETER_TAG};
use crate::common_alerting_protocol::resource::Resource;
use crate::common_alerting_protocol::utilities::*;
use crate::common_alerting_protocol::{Error, ParseEnumError, Result};
use chrono::prelude::*;
use chrono::DateTime;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str;
use std::str::FromStr;

pub const INFO_TAG: &[u8] = b"info";

const AUDIENCE_TAG: &[u8] = b"audience";
const CATEGORY_TAG: &[u8] = b"category";
const CERTAINTY_TAG: &[u8] = b"certainty";
const CONTACT_TAG: &[u8] = b"contact";
const DESCRIPTION_TAG: &[u8] = b"description";
const EFFECTIVE_TAG: &[u8] = b"effective";
const EVENT_CODE_TAG: &[u8] = b"eventCode";
const EVENT_TAG: &[u8] = b"event";
const EXPIRES_TAG: &[u8] = b"expires";
const HEADLINE_TAG: &[u8] = b"headline";
const INSTRUCTION_TAG: &[u8] = b"instruction";
const LANGUAGE_TAG: &[u8] = b"language";
const ONSET_TAG: &[u8] = b"onset";
const RESOURCE_TAG: &[u8] = b"resource";
const RESPONSE_TYPE_TAG: &[u8] = b"responseType";
const SENDER_NAME_TAG: &[u8] = b"senderName";
const SEVERITY_TAG: &[u8] = b"severity";
const URGENCY_TAG: &[u8] = b"urgency";
const WEB_TAG: &[u8] = b"web";

const CATEGORY_GEO: &str = "Geo";
const CATEGORY_MET: &str = "Met";
const CATEGORY_SAFETY: &str = "Safety";
const CATEGORY_SECURITY: &str = "Security";
const CATEGORY_RESCUE: &str = "Rescue";
const CATEGORY_FIRE: &str = "Fire";
const CATEGORY_HEALTH: &str = "Health";
const CATEGORY_ENV: &str = "Env";
const CATEGORY_TRANSPORT: &str = "Transport";
const CATEGORY_INFRA: &str = "Infra";
const CATEGORY_CBRNE: &str = "CBRNE";
const CATEGORY_OTHER: &str = "Other";

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Category {
    Geological,
    Meteorological,
    Safety,
    Security,
    Rescue,
    Fire,
    Health,
    Environmental,
    Transport,
    Infrastructure,
    CBRNE,
    Other,
}

impl FromStr for Category {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> std::result::Result<Category, ParseEnumError> {
        match enum_string {
            CATEGORY_GEO => Ok(Category::Geological),
            CATEGORY_MET => Ok(Category::Meteorological),
            CATEGORY_SAFETY => Ok(Category::Safety),
            CATEGORY_SECURITY => Ok(Category::Security),
            CATEGORY_RESCUE => Ok(Category::Rescue),
            CATEGORY_FIRE => Ok(Category::Fire),
            CATEGORY_HEALTH => Ok(Category::Health),
            CATEGORY_ENV => Ok(Category::Environmental),
            CATEGORY_TRANSPORT => Ok(Category::Transport),
            CATEGORY_INFRA => Ok(Category::Infrastructure),
            CATEGORY_CBRNE => Ok(Category::CBRNE),
            CATEGORY_OTHER => Ok(Category::Other),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Category::Geological => write!(f, "{}", CATEGORY_GEO),
            Category::Meteorological => write!(f, "{}", CATEGORY_MET),
            Category::Safety => write!(f, "{}", CATEGORY_SAFETY),
            Category::Security => write!(f, "{}", CATEGORY_SECURITY),
            Category::Rescue => write!(f, "{}", CATEGORY_RESCUE),
            Category::Fire => write!(f, "{}", CATEGORY_FIRE),
            Category::Health => write!(f, "{}", CATEGORY_HEALTH),
            Category::Environmental => write!(f, "{}", CATEGORY_ENV),
            Category::Transport => write!(f, "{}", CATEGORY_TRANSPORT),
            Category::Infrastructure => write!(f, "{}", CATEGORY_INFRA),
            Category::CBRNE => write!(f, "{}", CATEGORY_CBRNE),
            Category::Other => write!(f, "{}", CATEGORY_OTHER),
        }
    }
}

const URGENCY_IMMEDIATE: &str = "Immediate";
const URGENCY_EXPECTED: &str = "Expected";
const URGENCY_FUTURE: &str = "Future";
const URGENCY_PAST: &str = "Past";
const URGENCY_UNKNOWN: &str = "Unknown";

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Urgency {
    Immediate,
    Expected,
    Future,
    Past,
    Unknown,
}

impl FromStr for Urgency {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> std::result::Result<Urgency, ParseEnumError> {
        match enum_string {
            URGENCY_IMMEDIATE => Ok(Urgency::Immediate),
            URGENCY_EXPECTED => Ok(Urgency::Expected),
            URGENCY_FUTURE => Ok(Urgency::Future),
            URGENCY_PAST => Ok(Urgency::Past),
            URGENCY_UNKNOWN => Ok(Urgency::Unknown),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

impl Display for Urgency {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Urgency::Immediate => write!(f, "{}", URGENCY_IMMEDIATE),
            Urgency::Expected => write!(f, "{}", URGENCY_EXPECTED),
            Urgency::Future => write!(f, "{}", URGENCY_FUTURE),
            Urgency::Past => write!(f, "{}", URGENCY_PAST),
            Urgency::Unknown => write!(f, "{}", URGENCY_UNKNOWN),
        }
    }
}

const SEVERITY_EXTREME: &str = "Extreme";
const SEVERITY_SEVERE: &str = "Severe";
const SEVERITY_MODERATE: &str = "Moderate";
const SEVERITY_MINOR: &str = "Minor";
const SEVERITY_UNKNOWN: &str = "Unknown";

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Severity {
    Extreme,
    Severe,
    Moderate,
    Minor,
    Unknown,
}

impl FromStr for Severity {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> std::result::Result<Severity, ParseEnumError> {
        match enum_string {
            SEVERITY_EXTREME => Ok(Severity::Extreme),
            SEVERITY_SEVERE => Ok(Severity::Severe),
            SEVERITY_MODERATE => Ok(Severity::Moderate),
            SEVERITY_MINOR => Ok(Severity::Minor),
            SEVERITY_UNKNOWN => Ok(Severity::Unknown),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Severity::Extreme => write!(f, "{}", SEVERITY_EXTREME),
            Severity::Severe => write!(f, "{}", SEVERITY_SEVERE),
            Severity::Moderate => write!(f, "{}", SEVERITY_MODERATE),
            Severity::Minor => write!(f, "{}", SEVERITY_MINOR),
            Severity::Unknown => write!(f, "{}", SEVERITY_UNKNOWN),
        }
    }
}

const CERTAINTY_OBSERVED: &str = "Observed";
const CERTAINTY_VERYlIKELY: &str = "VeryLikely";
const CERTAINTY_LIKELY: &str = "Likely";
const CERTAINTY_POSSIBLE: &str = "Possible";
const CERTAINTY_UNLIKELY: &str = "Unlikely";
const CERTAINTY_UNKNOWN: &str = "Unknown";

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Certainty {
    Observed,
    VeryLikely,
    Likely,
    Possible,
    Unlikely,
    Unknown,
}

impl FromStr for Certainty {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> std::result::Result<Certainty, ParseEnumError> {
        match enum_string {
            CERTAINTY_OBSERVED => Ok(Certainty::Observed),
            CERTAINTY_VERYlIKELY => Ok(Certainty::VeryLikely),
            CERTAINTY_LIKELY => Ok(Certainty::Likely),
            CERTAINTY_POSSIBLE => Ok(Certainty::Possible),
            CERTAINTY_UNLIKELY => Ok(Certainty::Unlikely),
            CERTAINTY_UNKNOWN => Ok(Certainty::Unknown),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

impl Display for Certainty {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Certainty::Observed => write!(f, "{}", CERTAINTY_OBSERVED),
            Certainty::VeryLikely => write!(f, "{}", CERTAINTY_VERYlIKELY),
            Certainty::Likely => write!(f, "{}", CERTAINTY_LIKELY),
            Certainty::Possible => write!(f, "{}", CERTAINTY_POSSIBLE),
            Certainty::Unlikely => write!(f, "{}", CERTAINTY_UNLIKELY),
            Certainty::Unknown => write!(f, "{}", CERTAINTY_UNKNOWN),
        }
    }
}

const RESPONSE_TYPE_ALLCLEAR: &str = "AllClear";
const RESPONSE_TYPE_ASSESS: &str = "Assess";
const RESPONSE_TYPE_AVOID: &str = "Avoid";
const RESPONSE_TYPE_EVACUATE: &str = "Evacuate";
const RESPONSE_TYPE_EXECUTE: &str = "Execute";
const RESPONSE_TYPE_MONITOR: &str = "Monitor";
const RESPONSE_TYPE_NONE: &str = "None";
const RESPONSE_TYPE_PREPARE: &str = "Prepare";
const RESPONSE_TYPE_SHELTER: &str = "Shelter";

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ResponseType {
    AllClear,
    Assess,
    Avoid,
    Evacuate,
    Execute,
    Monitor,
    None,
    Prepare,
    Shelter,
}

impl FromStr for ResponseType {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> std::result::Result<ResponseType, ParseEnumError> {
        match enum_string {
            RESPONSE_TYPE_ALLCLEAR => Ok(ResponseType::AllClear),
            RESPONSE_TYPE_ASSESS => Ok(ResponseType::Assess),
            RESPONSE_TYPE_AVOID => Ok(ResponseType::Avoid),
            RESPONSE_TYPE_EVACUATE => Ok(ResponseType::Evacuate),
            RESPONSE_TYPE_EXECUTE => Ok(ResponseType::Execute),
            RESPONSE_TYPE_MONITOR => Ok(ResponseType::Monitor),
            RESPONSE_TYPE_NONE => Ok(ResponseType::None),
            RESPONSE_TYPE_PREPARE => Ok(ResponseType::Prepare),
            RESPONSE_TYPE_SHELTER => Ok(ResponseType::Shelter),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

impl Display for ResponseType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ResponseType::AllClear => write!(f, "{}", RESPONSE_TYPE_ALLCLEAR),
            ResponseType::Assess => write!(f, "{}", RESPONSE_TYPE_ASSESS),
            ResponseType::Avoid => write!(f, "{}", RESPONSE_TYPE_AVOID),
            ResponseType::Evacuate => write!(f, "{}", RESPONSE_TYPE_EVACUATE),
            ResponseType::Execute => write!(f, "{}", RESPONSE_TYPE_EXECUTE),
            ResponseType::Monitor => write!(f, "{}", RESPONSE_TYPE_MONITOR),
            ResponseType::None => write!(f, "{}", RESPONSE_TYPE_NONE),
            ResponseType::Prepare => write!(f, "{}", RESPONSE_TYPE_PREPARE),
            ResponseType::Shelter => write!(f, "{}", RESPONSE_TYPE_SHELTER),
        }
    }
}

pub const DEFAULT_LANGUAGE: &str = "en-US";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub areas: Vec<Area>,
    pub audience: Option<String>,
    pub categories: Vec<Category>,
    pub certainty: Option<Certainty>,
    pub contact: Option<String>,
    pub description: Option<String>,
    pub effective: Option<DateTime<FixedOffset>>,
    pub event_codes: Vec<EventCode>,
    pub event: Option<String>,
    pub expires: Option<DateTime<FixedOffset>>,
    pub headline: Option<String>,
    pub instruction: Option<String>,
    pub language: Option<String>,
    pub onset: Option<DateTime<FixedOffset>>,
    pub parameters: Vec<Parameter>,
    pub resources: Vec<Resource>,
    pub response_types: Vec<ResponseType>,
    pub sender_name: Option<String>,
    pub severity: Option<Severity>,
    pub urgency: Option<Urgency>,
    pub web: Option<String>,
}

impl Info {
    pub fn initialise() -> Info {
        return Info {
            areas: Vec::new(),
            audience: None,
            categories: Vec::new(),
            certainty: None,
            contact: None,
            description: None,
            effective: None,
            event_codes: Vec::new(),
            event: None,
            expires: None,
            headline: None,
            instruction: None,
            language: None,
            onset: None,
            parameters: Vec::new(),
            response_types: Vec::new(),
            resources: Vec::new(),
            sender_name: None,
            severity: None,
            urgency: None,
            web: None,
        };
    }

    pub fn deserialize_from_xml(namespace: &[u8], reader: &mut Reader<&[u8]>, buf: &mut std::vec::Vec<u8>, ns_buf: &mut std::vec::Vec<u8>) -> Result<Info> {
        let mut info = Info::initialise();

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(e)) if ns == namespace => match e.local_name() {
                    AREA_TAG => info.areas.push(Area::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    AUDIENCE_TAG => info.audience = Some(read_string(namespace, reader, buf, ns_buf, AUDIENCE_TAG)?),
                    CATEGORY_TAG => info
                        .categories
                        .push(read_string(namespace, reader, buf, ns_buf, CATEGORY_TAG)?.parse::<Category>()?),
                    CERTAINTY_TAG => info.certainty = Some(read_string(namespace, reader, buf, ns_buf, CERTAINTY_TAG)?.parse::<Certainty>()?),
                    CONTACT_TAG => info.contact = Some(read_string(namespace, reader, buf, ns_buf, CONTACT_TAG)?),
                    DESCRIPTION_TAG => info.description = Some(read_string(namespace, reader, buf, ns_buf, DESCRIPTION_TAG)?),
                    EFFECTIVE_TAG => info.effective = Some(DateTime::parse_from_rfc3339(&read_string(namespace, reader, buf, ns_buf, EFFECTIVE_TAG)?)?),
                    EVENT_CODE_TAG => info.event_codes.push(EventCode::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    EVENT_TAG => info.event = Some(read_string(namespace, reader, buf, ns_buf, EVENT_TAG)?),
                    EXPIRES_TAG => info.effective = Some(DateTime::parse_from_rfc3339(&read_string(namespace, reader, buf, ns_buf, EXPIRES_TAG)?)?),
                    HEADLINE_TAG => info.headline = Some(read_string(namespace, reader, buf, ns_buf, HEADLINE_TAG)?),
                    INSTRUCTION_TAG => info.instruction = Some(read_string(namespace, reader, buf, ns_buf, INSTRUCTION_TAG)?),
                    LANGUAGE_TAG => info.language = Some(read_string(namespace, reader, buf, ns_buf, LANGUAGE_TAG)?),
                    ONSET_TAG => info.effective = Some(DateTime::parse_from_rfc3339(&read_string(namespace, reader, buf, ns_buf, ONSET_TAG)?)?),
                    PARAMETER_TAG => info.parameters.push(Parameter::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    RESOURCE_TAG => info.resources.push(Resource::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    RESPONSE_TYPE_TAG => info
                        .response_types
                        .push(read_string(namespace, reader, buf, ns_buf, RESPONSE_TYPE_TAG)?.parse::<ResponseType>()?),
                    SENDER_NAME_TAG => info.sender_name = Some(read_string(namespace, reader, buf, ns_buf, SENDER_NAME_TAG)?),
                    SEVERITY_TAG => info.severity = Some(read_string(namespace, reader, buf, ns_buf, SEVERITY_TAG)?.parse::<Severity>()?),
                    URGENCY_TAG => info.urgency = Some(read_string(namespace, reader, buf, ns_buf, URGENCY_TAG)?.parse::<Urgency>()?),
                    WEB_TAG => info.web = Some(read_string(namespace, reader, buf, ns_buf, WEB_TAG)?),

                    unknown_tag => return Err(Error::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },

                (Some(ns), Event::End(ref e)) if ns == namespace => match e.local_name() {
                    INFO_TAG => return Ok(info),
                    AREA_TAG | AUDIENCE_TAG | CATEGORY_TAG | CERTAINTY_TAG | CONTACT_TAG | DESCRIPTION_TAG | EFFECTIVE_TAG | EVENT_CODE_TAG | EVENT_TAG
                    | EXPIRES_TAG | HEADLINE_TAG | INSTRUCTION_TAG | LANGUAGE_TAG | ONSET_TAG | PARAMETER_TAG | RESPONSE_TYPE_TAG | SENDER_NAME_TAG
                    | SEVERITY_TAG | URGENCY_TAG | WEB_TAG => (),

                    unknown_tag => return Err(Error::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },

                (_ns, _unknown_event) => (),
            }
        }
    }

    pub fn add_area<F>(&mut self, block: F)
    where
        F: Fn(&mut Area),
    {
        let mut area = Area::initialise();
        block(&mut area);
        self.areas.push(area);
    }

    pub fn add_event_code<F>(&mut self, block: F)
    where
        F: Fn(&mut EventCode),
    {
        let mut event_code = EventCode::initialise();
        block(&mut event_code);
        self.event_codes.push(event_code);
    }

    pub fn add_parameter<F>(&mut self, block: F)
    where
        F: Fn(&mut Parameter),
    {
        let mut parameter = Parameter::initialise();
        block(&mut parameter);
        self.parameters.push(parameter);
    }

    pub fn add_resource<F>(&mut self, block: F)
    where
        F: Fn(&mut Resource),
    {
        let mut resource = Resource::initialise();
        block(&mut resource);
        self.resources.push(resource);
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::VERSION_1_2;
    use crate::common_alerting_protocol::info::Info;
    use quick_xml::Reader;

    use std::str::FromStr;

    #[test]
    fn text_parse_from_xml_1() {
        let xml = r#"<info xmlns="urn:oasis:names:tc:emergency:cap:1.2">
     <category>Geo</category>
     <event>Earthquake</event>
     <urgency>Past</urgency>
     <severity>Minor</severity>
     <certainty>Observed</certainty>
     <senderName>Southern California Seismic Network (TriNet) operated by Caltech and USGS</senderName>
     <headline>EQ 3.4 Imperial County CA</headline>
     <description>A minor earthquake measuring 3.4 on the Richter scale occurred near Brawley, California at 8:30 PM Pacific Daylight Time on Wednesday, June 11, 2003. (This event has now been reviewed by a seismologist)</description>
     <web>http://www.trinet.org/scsn/scsn.html</web>
     <parameter>
       <valueName>EventID</valueName>
       <value>13970876</value>
     </parameter>
     <parameter>
       <valueName>Version</valueName>
       <value>1</value>
     </parameter>
     <parameter>
       <valueName>Magnitude</valueName>
       <value>3.4 Ml</value>
     </parameter>
     <parameter>
       <valueName>Depth</valueName>
       <value>11.8 mi.</value>
     </parameter>
     <parameter>
       <valueName>Quality</valueName>
       <value>Excellent</value>
     </parameter>
     <area>
       <areaDesc>1 mi. WSW of Brawley, CA; 11 mi. N of El Centro, CA; 30 mi. E of OCOTILLO (quarry); 1 mi. N of the Imperial Fault</areaDesc>
       <circle>32.9525,-115.5527 0</circle>
     </area>
   </info>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);
        let info = Info::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();

        assert_eq!(Some(String::from_str("Earthquake").unwrap()), info.event);
        assert_eq!(Some(String::from_str("EQ 3.4 Imperial County CA").unwrap()), info.headline);

        assert_eq!(5, info.parameters.len());
        assert_eq!(1, info.areas.len());
    }

    #[test]
    fn test_parse_xml_2() {
        let xml = r#"<info xmlns="urn:oasis:names:tc:emergency:cap:1.2">
     <category>Met</category>
     <event>SEVERE THUNDERSTORM</event>
     <responseType>Shelter</responseType>
     <urgency>Immediate</urgency>
     <severity>Severe</severity>
     <certainty>Observed</certainty>
     <eventCode>
       <valueName>SAME</valueName>
       <value>SVR</value>
     </eventCode>
     <expires>2003-06-17T16:00:00-07:00</expires>
     <senderName>NATIONAL WEATHER SERVICE SACRAMENTO CA</senderName>
     <headline>SEVERE THUNDERSTORM WARNING</headline>
     <description> AT 254 PM PDT...NATIONAL WEATHER SERVICE DOPPLER RADAR INDICATED A SEVERE THUNDERSTORM OVER SOUTH CENTRAL ALPINE COUNTY...OR ABOUT 18 MILES SOUTHEAST OF KIRKWOOD...MOVING SOUTHWEST AT 5 MPH. HAIL...INTENSE RAIN AND STRONG DAMAGING WINDS ARE LIKELY WITH THIS STORM.</description>
     <instruction>TAKE COVER IN A SUBSTANTIAL SHELTER UNTIL THE STORM PASSES.</instruction>
     <contact>BARUFFALDI/JUSKIE</contact>
     <area>
       <areaDesc>EXTREME NORTH CENTRAL TUOLUMNE COUNTY IN CALIFORNIA, EXTREME NORTHEASTERN CALAVERAS COUNTY IN CALIFORNIA, SOUTHWESTERN ALPINE COUNTY IN CALIFORNIA</areaDesc>
       <polygon>38.47,-120.14 38.34,-119.95 38.52,-119.74 38.62,-119.89 38.47,-120.14</polygon>
       <geocode>
         <valueName>SAME</valueName>
         <value>006109</value>
       </geocode>
       <geocode>
         <valueName>SAME</valueName>
         <value>006009</value>
       </geocode>
       <geocode>
         <valueName>SAME</valueName>
         <value>006003</value>
       </geocode>
     </area>
   </info>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);
        Info::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();
    }
}

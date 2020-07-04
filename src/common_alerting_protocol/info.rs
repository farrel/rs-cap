use crate::common_alerting_protocol::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
use crate::common_alerting_protocol::area::{Area, AREA_TAG};
use crate::common_alerting_protocol::deserialise_error::{DeserialiseError, ParseEnumError};
use crate::common_alerting_protocol::event_code::EventCode;
use crate::common_alerting_protocol::parameter::{Parameter, PARAMETER_TAG};
use crate::common_alerting_protocol::utilities::*;
use chrono::prelude::*;
use chrono::DateTime;
use quick_xml::events::Event;
use quick_xml::Reader;
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
const RESPONSE_TYPE_TAG: &[u8] = b"responseType";
const SENDER_NAME_TAG: &[u8] = b"senderName";
const SEVERITY_TAG: &[u8] = b"severity";
const URGENCY_TAG: &[u8] = b"urgency";
const WEB_TAG: &[u8] = b"web";

enum Category {
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

    fn from_str(enum_string: &str) -> Result<Category, ParseEnumError> {
        match enum_string {
            "Geo" => Ok(Category::Geological),
            "Met" => Ok(Category::Meteorological),
            "Safety" => Ok(Category::Safety),
            "Security" => Ok(Category::Security),
            "Rescue" => Ok(Category::Rescue),
            "Fire" => Ok(Category::Fire),
            "Health" => Ok(Category::Health),
            "Env" => Ok(Category::Environmental),
            "Transport" => Ok(Category::Transport),
            "Infra" => Ok(Category::Infrastructure),
            "CBRNE" => Ok(Category::CBRNE),
            "Other" => Ok(Category::Other),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

enum Urgency {
    Immediate,
    Expected,
    Future,
    Past,
    Unknown,
}

impl FromStr for Urgency {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> Result<Urgency, ParseEnumError> {
        match enum_string {
            "Immediate" => Ok(Urgency::Immediate),
            "Expected" => Ok(Urgency::Expected),
            "Future" => Ok(Urgency::Future),
            "Past" => Ok(Urgency::Past),
            "Unknown" => Ok(Urgency::Unknown),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

enum Severity {
    Extreme,
    Severe,
    Moderate,
    Minor,
    Unknown,
}

impl FromStr for Severity {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> Result<Severity, ParseEnumError> {
        match enum_string {
            "Extreme" => Ok(Severity::Extreme),
            "Severe" => Ok(Severity::Severe),
            "Moderate" => Ok(Severity::Moderate),
            "Minor" => Ok(Severity::Minor),
            "Unknown" => Ok(Severity::Unknown),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

enum Certainty {
    Observed,
    VeryLikely,
    Likely,
    Possible,
    Unlikely,
    Unknown,
}

impl FromStr for Certainty {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> Result<Certainty, ParseEnumError> {
        match enum_string {
            "Observed" => Ok(Certainty::Observed),
            "VeryLikely" => Ok(Certainty::VeryLikely),
            "Likely" => Ok(Certainty::Likely),
            "Possible" => Ok(Certainty::Possible),
            "Unlikely" => Ok(Certainty::Unlikely),
            "Unknown" => Ok(Certainty::Unknown),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

enum ResponseType {
    Shelter,
    Evacuate,
    Prepare,
    Execute,
    Monitor,
    Assess,
    None,
}

impl FromStr for ResponseType {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> Result<ResponseType, ParseEnumError> {
        match enum_string {
            "Shelter" => Ok(ResponseType::Shelter),
            "Evacuate" => Ok(ResponseType::Evacuate),
            "Prepare" => Ok(ResponseType::Prepare),
            "Execute" => Ok(ResponseType::Execute),
            "Monitor" => Ok(ResponseType::Monitor),
            "Assess" => Ok(ResponseType::Assess),
            "None" => Ok(ResponseType::None),
            _ => Err(ParseEnumError::enum_not_found(enum_string)),
        }
    }
}

pub const DEFAULT_LANGUAGE: &str = "en-US";

pub struct Info {
    areas: Vec<Area>,
    audience: Option<String>,
    categories: Vec<Category>,
    certainty: Option<Certainty>,
    contact: Option<String>,
    description: Option<String>,
    effective: Option<DateTime<FixedOffset>>,
    event_codes: Vec<EventCode>,
    event: Option<String>,
    expires: Option<DateTime<FixedOffset>>,
    headline: Option<String>,
    instruction: Option<String>,
    language: Option<String>,
    onset: Option<DateTime<FixedOffset>>,
    parameters: Vec<Parameter>,
    response_types: Vec<ResponseType>,
    sender_name: Option<String>,
    severity: Option<Severity>,
    urgency: Option<Urgency>,
    web: Option<String>,
}

impl Info {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Info, DeserialiseError> {
        let mut info = Info {
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
            sender_name: None,
            severity: None,
            urgency: None,
            web: None,
        };

        let vec = &mut Vec::new();

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(ref e)) if ns == namespace => match e.local_name() {
                    AREA_TAG => info.areas.push(Area::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    AUDIENCE_TAG => info.audience = Some(reader.read_text(AUDIENCE_TAG, vec)?),
                    CATEGORY_TAG => info.categories.push(reader.read_text(CATEGORY_TAG, vec)?.parse::<Category>()?),
                    CERTAINTY_TAG => info.certainty = Some(reader.read_text(CERTAINTY_TAG, vec)?.parse::<Certainty>()?),
                    CONTACT_TAG => info.contact = Some(reader.read_text(CONTACT_TAG, vec)?),
                    DESCRIPTION_TAG => info.description = Some(reader.read_text(DESCRIPTION_TAG, vec)?),
                    EFFECTIVE_TAG => info.effective = Some(DateTime::parse_from_rfc3339(&reader.read_text(EFFECTIVE_TAG, vec)?)?),
                    EVENT_CODE_TAG => info.event_codes.push(EventCode::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    EVENT_TAG => info.event = Some(reader.read_text(EVENT_TAG, vec)?),

                    EXPIRES_TAG => info.effective = Some(DateTime::parse_from_rfc3339(&reader.read_text(EXPIRES_TAG, vec)?)?),
                    HEADLINE_TAG => info.headline = Some(reader.read_text(HEADLINE_TAG, vec)?),
                    INSTRUCTION_TAG => info.instruction = Some(reader.read_text(INSTRUCTION_TAG, vec)?),
                    LANGUAGE_TAG => info.language = Some(reader.read_text(LANGUAGE_TAG, vec)?),
                    ONSET_TAG => info.effective = Some(DateTime::parse_from_rfc3339(&reader.read_text(ONSET_TAG, vec)?)?),
                    PARAMETER_TAG => info.parameters.push(Parameter::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    RESPONSE_TYPE_TAG => info.response_types.push(reader.read_text(RESPONSE_TYPE_TAG, vec)?.parse::<ResponseType>()?),
                    SENDER_NAME_TAG => info.sender_name = Some(reader.read_text(SENDER_NAME_TAG, vec)?),
                    SEVERITY_TAG => info.severity = Some(reader.read_text(SEVERITY_TAG, vec)?.parse::<Severity>()?),
                    URGENCY_TAG => info.urgency = Some(reader.read_text(URGENCY_TAG, vec)?.parse::<Urgency>()?),
                    WEB_TAG => info.web = Some(reader.read_text(WEB_TAG, vec)?),

                    unknown_tag => return Err(DeserialiseError::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },

                (Some(ns), Event::End(ref e)) if ns == namespace => match e.local_name() {
                    INFO_TAG => return Ok(info),
                    AREA_TAG | AUDIENCE_TAG | CATEGORY_TAG | CERTAINTY_TAG | CONTACT_TAG | DESCRIPTION_TAG | EFFECTIVE_TAG | EVENT_CODE_TAG | EVENT_TAG
                    | EXPIRES_TAG | HEADLINE_TAG | INSTRUCTION_TAG | LANGUAGE_TAG | ONSET_TAG | PARAMETER_TAG | RESPONSE_TYPE_TAG | SENDER_NAME_TAG
                    | SEVERITY_TAG | URGENCY_TAG | WEB_TAG => (),

                    unknown_tag => return Err(DeserialiseError::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },

                (_ns, unknown_event) => return Err(DeserialiseError::unknown_event(unknown_event)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
    use crate::common_alerting_protocol::info::Info;
    use quick_xml::Reader;
    use std::str;
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
        let info = Info::deserialize_from_xml(VERSION_1_2, reader, &mut buf, &mut ns_buf).unwrap();

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
        let info = Info::deserialize_from_xml(VERSION_1_2, reader, &mut buf, &mut ns_buf).unwrap();
    }
}

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

const AUDIENCE_TAG: &str = "audience";
const CATEGORY_TAG: &str = "category";
const CERTAINTY_TAG: &str = "certainty";
const CONTACT_TAG: &str = "contact";
const DESCRIPTION_TAG: &str = "description";
const EFFECTIVE_TAG: &str = "effective";
const EVENT_CODE_TAG: &str = "eventCode";
const EVENT_TAG: &str = "event";
const EXPIRES_TAG: &str = "expires";
const HEADLINE_TAG: &str = "headline";
const INFO_TAG: &str = "info";
const INSTRUCTION_TAG: &str = "instruction";
const LANGUAGE_TAG: &str = "language";
const ONSET_TAG: &str = "onset";
const RESPONSE_TYPE_TAG: &str = "responseType";
const SENDER_NAME_TAG: &str = "senderName";
const SEVERITY_TAG: &str = "severity";
const URGENCY_TAG: &str = "urgency";
const WEB_TAG: &str = "web";

enum Category {
    Geo,
    Met,
    Safety,
    Security,
    Rescue,
    Fire,
    Health,
    Env,
    Transport,
    Infra,
    CBRNE,
    Other,
}

impl FromStr for Category {
    type Err = ParseEnumError;

    fn from_str(enum_string: &str) -> Result<Category, ParseEnumError> {
        match enum_string {
            "Geo" => Ok(Category::Geo),
            "Met" => Ok(Category::Met),
            "Safety" => Ok(Category::Safety),
            "Security" => Ok(Category::Security),
            "Rescue" => Ok(Category::Rescue),
            "Fire" => Ok(Category::Fire),
            "Health" => Ok(Category::Health),
            "Env" => Ok(Category::Env),
            "Transport" => Ok(Category::Transport),
            "Infra" => Ok(Category::Infra),
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
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Info, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

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

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (ref _ns, Event::Start(ref e)) => match str::from_utf8(e.name())? {
                    INFO_TAG => (),

                    AREA_TAG => info.areas.push(Area::deserialize_from_xml(reader)?),
                    AUDIENCE_TAG => info.audience = Some(parse_string(reader, AUDIENCE_TAG)?),
                    CATEGORY_TAG => info.categories.push(parse_string(reader, CATEGORY_TAG)?.parse::<Category>()?),
                    CERTAINTY_TAG => info.certainty = Some(parse_string(reader, CERTAINTY_TAG)?.parse::<Certainty>()?),
                    CONTACT_TAG => info.contact = Some(parse_string(reader, CONTACT_TAG)?),
                    DESCRIPTION_TAG => info.description = Some(parse_string(reader, DESCRIPTION_TAG)?),
                    EFFECTIVE_TAG => info.effective = Some(parse_datetime(reader, EFFECTIVE_TAG)?),
                    EVENT_CODE_TAG => info.event_codes.push(EventCode::deserialize_from_xml(reader)?),
                    EVENT_TAG => info.event = Some(String::from(&parse_string(reader, EVENT_TAG)?)),
                    EXPIRES_TAG => info.effective = Some(parse_datetime(reader, EXPIRES_TAG)?),
                    HEADLINE_TAG => info.headline = Some(parse_string(reader, HEADLINE_TAG)?),
                    INSTRUCTION_TAG => info.instruction = Some(parse_string(reader, INSTRUCTION_TAG)?),
                    LANGUAGE_TAG => info.language = Some(parse_string(reader, LANGUAGE_TAG)?),
                    ONSET_TAG => info.effective = Some(parse_datetime(reader, ONSET_TAG)?),
                    PARAMETER_TAG => info.parameters.push(Parameter::deserialize_from_xml(reader)?),
                    RESPONSE_TYPE_TAG => info.response_types.push(parse_string(reader, RESPONSE_TYPE_TAG)?.parse::<ResponseType>()?),
                    SENDER_NAME_TAG => info.sender_name = Some(parse_string(reader, SENDER_NAME_TAG)?),
                    SEVERITY_TAG => info.severity = Some(parse_string(reader, SEVERITY_TAG)?.parse::<Severity>()?),
                    URGENCY_TAG => info.urgency = Some(parse_string(reader, URGENCY_TAG)?.parse::<Urgency>()?),
                    WEB_TAG => info.web = Some(parse_string(reader, WEB_TAG)?),

                    unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
                },
                (ref _ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                    INFO_TAG => return Ok(info),
                    _ => (),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::info::Info;
    use quick_xml::Reader;

    #[test]
    fn text_parse_from_xml_1() {
        let xml = r#"<info>
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

        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        let info = Info::deserialize_from_xml(reader).unwrap();

        assert_eq!("Earthquake", info.event.unwrap());
        assert_eq!("EQ 3.4 Imperial County CA", info.headline.unwrap());

        assert_eq!(5, info.parameters.len());
        assert_eq!(1, info.areas.len());
    }

    #[test]
    fn test_parse_xml_2() {
        let xml = r#"<info>
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

        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        let info = Info::deserialize_from_xml(reader).unwrap();
    }
}

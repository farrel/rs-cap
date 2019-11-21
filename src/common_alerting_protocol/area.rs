use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;

use crate::common_alerting_protocol::circle::{Circle, CIRCLE_TAG};
use crate::common_alerting_protocol::geocode::{Geocode, GEOCODE_TAG};
use crate::common_alerting_protocol::polygon::{Polygon, POLYGON_TAG};

pub struct Area {
    area_desc: String,
    altitude: Option<f64>,
    ceiling: Option<f64>,
    circles: Vec<Circle>,
    geocodes: Vec<Geocode>,
    polygons: Vec<Polygon>,
}

pub const AREA_TAG: &str = "area";
const AREA_DESC_TAG: &str = "areaDesc";
const ALTITUDE_TAG: &str = "altitude";
const CEILING_TAG: &str = "ceiling";

impl Area {
    pub fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Area, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        let mut area = Area {
            area_desc: String::new(),
            altitude: None,
            ceiling: None,
            circles: Vec::new(),
            polygons: Vec::new(),
            geocodes: Vec::new(),
        };

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (_ns, Event::Start(ref e)) => match str::from_utf8(e.name())? {
                    AREA_TAG => (),
                    AREA_DESC_TAG => area.area_desc.push_str(&parse_string(reader, AREA_DESC_TAG)?),
                    POLYGON_TAG => area.polygons.push(Polygon::deserialize_from_xml(reader)?),
                    GEOCODE_TAG => area.geocodes.push(Geocode::deserialize_from_xml(reader)?),
                    ALTITUDE_TAG => area.altitude = parse_f64(reader, CEILING_TAG)?,
                    CEILING_TAG => area.ceiling = parse_f64(reader, CEILING_TAG)?,
                    unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
                },
                (_ns, Event::Text(e)) => (),
                (_ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                    AREA_TAG => return Ok(area),
                    AREA_DESC_TAG | POLYGON_TAG | GEOCODE_TAG | CIRCLE_TAG => (),
                    unknown_tag => return Err(DeserialiseError::tag_not_expected(unknown_tag)),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::area::Area;
    use quick_xml::Reader;

    const AREA_XML: &str = "<area>
      <areaDesc>City of Thunder Bay</areaDesc>
      <altitude>100</altitude>
      <ceiling>200</ceiling>
      <polygon>
        48.5448,-89.0388 48.5001,-89.0231 48.4482,-89.0071 48.3079,-89.1102
        48.3096,-89.1933 48.3065,-89.2028 48.3101,-89.2143 48.3119,-89.271
        48.334,-89.3597 48.3528,-89.3957 48.3838,-89.4298 48.4198,-89.4449
        48.484,-89.432 48.519,-89.4061 48.551,-89.3458 48.59,-89.1847
        48.5859,-89.1228 48.568,-89.0639 48.5448,-89.0388
      </polygon>
      <geocode>
        <valueName>layer:EC-MSC-SMC:1.0:CLC</valueName>
        <value>048100</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558003</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558004</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558011</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558028</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558090</value>
      </geocode>
    </area>";

    #[test]
    fn deserialize_from_xml() {
        let reader = &mut Reader::from_str(AREA_XML);
        let area = Area::deserialize_from_xml(reader).unwrap();

        assert_eq!("City of Thunder Bay", area.area_desc);
        assert_eq!(1, area.polygons.len());
        assert_eq!(6, area.geocodes.len());
        assert_eq!(Some(100.0), area.altitude);
        assert_eq!(Some(200.0), area.ceiling);
    }
}

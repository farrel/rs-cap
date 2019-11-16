use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialize_from_xml::{DeserialiseError, DeserializeFromXml};
use crate::common_alerting_protocol::point::Point;
use crate::common_alerting_protocol::utilities::*;

const POLYGON_TAG: &str = "polygon";

pub struct Polygon {
    points: Vec<Point>,
}

impl DeserializeFromXml for Polygon {
    fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Box<Polygon>, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
                Ok((ref _ns, Event::Start(ref e))) => match str::from_utf8(e.name())? {
                    POLYGON_TAG => (),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                Ok((ref _ns, Event::Text(ref e))) => {
                    return Ok(Box::new(Polygon {
                        points: Point::parse_points_string(&e.unescape_and_decode(reader)?),
                    }));
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::deserialize_from_xml::DeserializeFromXml;
    use crate::common_alerting_protocol::polygon::Polygon;
    use quick_xml::Reader;

    fn test_deserialise_from_xml() {
        let xml = "<polygon>48.5448,-89.0388 48.5001,-89.0231 48.4482,-89.0071 48.3079,-89.1102
        48.3096,-89.1933 48.3065,-89.2028 48.3101,-89.2143 48.3119,-89.271
        48.334,-89.3597 48.3528,-89.3957 48.3838,-89.4298 48.4198,-89.4449
        48.484,-89.432 48.519,-89.4061 48.551,-89.3458 48.59,-89.1847
        48.5859,-89.1228 48.568,-89.0639 48.5448,-89.0388
      </polygon>";

        let reader = &mut Reader::from_str(xml);
        let polygon = Polygon::deserialize_from_xml(reader).unwrap();
        assert_eq!(19, polygon.points.len());
    }
}

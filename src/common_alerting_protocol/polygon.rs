use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::point::Point;
use crate::common_alerting_protocol::utilities::*;

pub const POLYGON_TAG: &str = "polygon";

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Polygon, DeserialiseError> {
        return Ok(Polygon {
            points: Point::parse_points_string(reader.read_text(POLYGON_TAG, &mut Vec::new())?.as_str())?,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::{VERSION_1_0, VERSION_1_1, VERSION_1_2};
    use crate::common_alerting_protocol::polygon::Polygon;
    use quick_xml::Reader;

    #[test]
    fn test_deserialise_from_xml() {
        let xml = r#"<polygon xmlns="urn:oasis:names:tc:emergency:cap:1.2">48.5448,-89.0388 48.5001,-89.0231 48.4482,-89.0071 48.3079,-89.1102
        48.3096,-89.1933 48.3065,-89.2028 48.3101,-89.2143 48.3119,-89.271
        48.334,-89.3597 48.3528,-89.3957 48.3838,-89.4298 48.4198,-89.4449
        48.484,-89.432 48.519,-89.4061 48.551,-89.3458 48.59,-89.1847
        48.5859,-89.1228 48.568,-89.0639 48.5448,-89.0388
      </polygon>"#;

        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);

        let mut polygon = Polygon::deserialize_from_xml(VERSION_1_2, reader, &mut buf, &mut ns_buf).unwrap();
        assert_eq!(19, polygon.points.len());
        let point = polygon.points.pop().unwrap();
        assert_eq!(48.5448, point.latitude);
        assert_eq!(-89.0388, point.longitude);
    }
}

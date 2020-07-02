use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

pub const CIRCLE_TAG: &[u8] = b"circle";

pub struct Circle {
    latitude: f64,
    longitude: f64,
    radius: f64,
}

pub fn split_circle_string(circle_string: &str) -> Result<(f64, f64, f64), DeserialiseError> {
    let mut point_and_radius = circle_string.split(' ');

    if let (Some(point_string), Some(radius)) = (point_and_radius.next(), point_and_radius.next()) {
        let mut coords = point_string.split(',');

        if let (Some(latitude), Some(longitude)) = (coords.next(), coords.next()) {
            Ok((latitude.parse::<f64>()?, longitude.parse::<f64>()?, radius.parse::<f64>()?))
        } else {
            Err(DeserialiseError::error(&format!("Could not parse {}", point_string)))
        }
    } else {
        Err(DeserialiseError::error(&format!("Could not split points and radius {}", circle_string)))
    }
}

impl Circle {
    pub fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Circle, DeserialiseError> {
        let (latitude, longitude, radius) = split_circle_string(reader.read_text(CIRCLE_TAG, &mut Vec::new())?.as_str())?;

        return Ok(Circle {
            latitude: latitude,
            longitude: longitude,
            radius: radius,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::circle::Circle;
    use quick_xml::Reader;

    #[test]
    fn deserialize_from_xml() {
        let xml = r#"<circle xmlns="urn:oasis:names:tc:emergency:cap:1.2">80,20.7 10.5</circle>"#;
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf).unwrap();

        let circle = Circle::deserialize_from_xml(reader).unwrap();

        assert_eq!(80.0, circle.latitude);
        assert_eq!(20.7, circle.longitude);
        assert_eq!(10.5, circle.radius);
    }
}

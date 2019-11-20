use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;

pub const CIRCLE_TAG: &str = "circle";

pub struct Circle {
    latitude: f64,
    longitude: f64,
    radius: f64,
}

pub fn split_circle_string(circle_string: &str) -> Result<(&str, &str, &str), DeserialiseError> {
    let mut point_and_radius = circle_string.split(' ');

    if let (Some(point_string), Some(radius)) = (point_and_radius.next(), point_and_radius.next()) {
        let mut coords = point_string.split(',');

        if let (Some(latitude), Some(longitude)) = (coords.next(), coords.next()) {
            Ok((latitude, longitude, radius))
        } else {
            Err(DeserialiseError::error(&format!("Could not parse {}", point_string)))
        }
    } else {
        Err(DeserialiseError::error(&format!("Could not split points and radius {}", circle_string)))
    }
}

impl Circle {
    pub fn deserialize_from_xml(reader: &mut Reader<&[u8]>) -> Result<Circle, DeserialiseError> {
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();

        let mut latitude_string = String::new();
        let mut longitude_string = String::new();
        let mut radius_string = String::new();

        loop {
            match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
                (_ns, Event::Start(ref e)) => match str::from_utf8(e.name())? {
                    CIRCLE_TAG => (),
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                (_ns, Event::Text(e)) => {
                    let circle_string = &e.unescape_and_decode(reader)?;
                    let (latitude, longitude, radius) = split_circle_string(circle_string)?;
                    latitude_string.push_str(latitude);
                    longitude_string.push_str(longitude);
                    radius_string.push_str(radius);
                }

                (_ns, Event::End(ref e)) => match str::from_utf8(e.name())? {
                    CIRCLE_TAG => {
                        return Ok(Circle {
                            latitude: latitude_string.parse::<f64>()?,
                            longitude: longitude_string.parse::<f64>()?,
                            radius: radius_string.parse::<f64>()?,
                        })
                    }
                    unknown_tag => return Err(DeserialiseError::tag_not_recognised(unknown_tag)),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::circle::Circle;
    use quick_xml::Reader;

    #[test]
    fn initialise() {
        let circle = Circle {
            latitude: 0.0,
            longitude: 0.0,
            radius: 5.0,
        };

        assert_eq!(0.0, circle.latitude);
        assert_eq!(0.0, circle.longitude);
        assert_eq!(5.0, circle.radius);
    }

    #[test]
    fn deserialize_from_xml() {
        let xml = r#"<circle>80,20.7 10.5</circle>"#;
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);

        let circle = Circle::deserialize_from_xml(reader).unwrap();

        assert_eq!(80.0, circle.latitude);
        assert_eq!(20.7, circle.longitude);
        assert_eq!(10.5, circle.radius);
    }
}

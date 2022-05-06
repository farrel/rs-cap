use crate::error::Error;
use crate::result::Result;
use crate::utilities::read_string;
use geo::Point;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::str;

pub const CIRCLE_TAG: &[u8] = b"circle";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Circle {
    location: Option<Point<f64>>,
    radius: Option<f64>,
}

pub fn split_circle_string(circle_string: &str) -> Result<(f64, f64, f64)> {
    let mut point_and_radius = circle_string.split(' ');

    if let (Some(point_string), Some(radius)) = (point_and_radius.next(), point_and_radius.next()) {
        let mut coords = point_string.split(',');

        if let (Some(latitude), Some(longitude)) = (coords.next(), coords.next()) {
            Ok((latitude.parse::<f64>()?, longitude.parse::<f64>()?, radius.parse::<f64>()?))
        } else {
            Err(Error::Other(format!("Could not parse {}", point_string)))
        }
    } else {
        Err(Error::Other(format!("Could not split points and radius {}", circle_string)))
    }
}

impl Circle {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Option<Circle>> {
        match read_string(namespace, reader, buf, ns_buf, CIRCLE_TAG)?.and_then(|string| split_circle_string(&string).ok()) {
            Some((latitude, longitude, radius)) => Ok(Some(Circle {
                location: Some(Point::new(longitude, latitude)),
                radius: Some(radius),
            })),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alert::VERSION_1_2;
    use crate::circle::Circle;
    use quick_xml::Reader;

    #[test]
    fn deserialize_from_xml() {
        let xml = r#"<circle xmlns="urn:oasis:names:tc:emergency:cap:1.2">80,20.7 10.5</circle>"#;
        let buf = &mut Vec::new();
        let ns_buf = &mut Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(buf, ns_buf).unwrap();

        let circle = Circle::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, buf, ns_buf).unwrap().unwrap();

        assert_eq!(80.0, circle.location.unwrap().lat());
        assert_eq!(20.7, circle.location.unwrap().lng());
        assert_eq!(Some(10.5), circle.radius);
    }
}

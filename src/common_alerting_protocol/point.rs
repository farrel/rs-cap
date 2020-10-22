use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;
use serde::{Deserialize, Serialize};

const MAX_LONGITUDE: f64 = 180.0;
const MIN_LONGITUDE: f64 = -180.0;

const MAX_LATITUDE: f64 = 90.0;
const MIN_LATITUDE: f64 = -90.0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Point {
    pub fn initialise() -> Point {
        Point {
            latitude: None,
            longitude: None,
        }
    }

    pub fn parse_point_string(point_string: &str) -> DeserialiseResult<Point> {
        let mut coords = point_string.split(',');

        if let (Some(latitude), Some(longitude)) = (coords.next(), coords.next()) {
            Ok(Point {
                latitude: Some(latitude.parse::<f64>()?),
                longitude: Some(longitude.parse::<f64>()?),
            })
        } else {
            Err(DeserialiseError::error(&format!("Error parsing point string: {}", point_string)))
        }
    }

    pub fn parse_points_string(points_string: &str) -> Result<Vec<Point>, DeserialiseError> {
        let point_strings: Vec<&str> = points_string.split_whitespace().collect();
        let mut points = Vec::new();

        for point_string in point_strings.iter() {
            points.push(Point::parse_point_string(point_string)?);
        }

        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::point::Point;

    #[test]
    fn parse_points_string() {
        let mut points = Point::parse_points_string("48.0,-89.0 48,-89").unwrap();
        assert_eq!(2, points.len());

        if let Some(point) = points.pop() {
            assert_eq!(48.0, point.latitude.unwrap());
            assert_eq!(-89.0, point.longitude.unwrap());
        } else {
            panic!("No points parsed");
        }

        if let Some(point) = points.pop() {
            assert_eq!(48.0, point.latitude.unwrap());
            assert_eq!(-89.0, point.longitude.unwrap());
        } else {
            panic!("No points parsed");
        }
    }
}

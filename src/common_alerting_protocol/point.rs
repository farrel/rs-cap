use crate::common_alerting_protocol::deserialise_error::DeserialiseError;

const MAX_LONGITUDE: f64 = 180.0;
const MIN_LONGITUDE: f64 = -180.0;

const MAX_LATITUDE: f64 = 90.0;
const MIN_LATITUDE: f64 = 90.0;

#[derive(Debug)]
pub struct Point {
    latitude: f64,
    longitude: f64,
}

impl Point {
    pub fn parse_point_string(point_string: &str) -> Result<Point, DeserialiseError> {
        let mut coords = point_string.split(',');

        if let (Some(latitude), Some(longitude)) = (coords.nth(0), coords.nth(1)) {
            Ok(Point {
                latitude: latitude.parse::<f64>()?,
                longitude: longitude.parse::<f64>()?,
            })
        } else {
            Err(DeserialiseError::error(&format!("Error parsing points string: {}", point_string)))
        }
    }

    pub fn parse_points_string(points_string: &str) -> Result<Vec<Point>, DeserialiseError> {
        let point_strings: Vec<&str> = points_string.split(' ').collect();
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
    fn initialise() {
        let point = Point { latitude: 0.0, longitude: 0.0 };
        assert_eq!(0.0, point.latitude);
        assert_eq!(0.0, point.longitude);
    }
}

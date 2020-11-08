use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::utilities::*;

const MAX_LONGITUDE: f64 = 180.0;
const MIN_LONGITUDE: f64 = -180.0;

const MAX_LATITUDE: f64 = 90.0;
const MIN_LATITUDE: f64 = -90.0;

pub fn parse_point_string(point_string: &str) -> DeserialiseResult<(f64, f64)> {
    let mut coords = point_string.split(',');

    if let (Some(latitude), Some(longitude)) = (coords.next(), coords.next()) {
        return Ok((latitude.parse::<f64>()?, longitude.parse::<f64>()?));
    } else {
        return Err(DeserialiseError::error(&format!("Error parsing point string: {}", point_string)));
    }
}

pub fn parse_points_string(points_string: &str) -> DeserialiseResult<Vec<(f64, f64)>> {
    let point_strings: Vec<&str> = points_string.split_whitespace().collect();
    let mut points = Vec::new();

    for point_string in point_strings.iter() {
        points.push(parse_point_string(point_string)?);
    }

    return Ok(points);
}

#[test]
fn test_parse_points_string() {
    let mut points = parse_points_string("48.0,-89.0 48,-89").unwrap();
    assert_eq!(2, points.len());

    if let Some(point) = points.pop() {
        assert_eq!((48.0, -89.0), point);
    } else {
        panic!("No points parsed");
    }

    if let Some(point) = points.pop() {
        assert_eq!((48.0, -89.0), point);
    } else {
        panic!("No points parsed");
    }
}

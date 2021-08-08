use crate::common_alerting_protocol::{Error, Result};

const MAX_LONGITUDE: f64 = 180.0;
const MIN_LONGITUDE: f64 = -180.0;

const MAX_LATITUDE: f64 = 90.0;
const MIN_LATITUDE: f64 = -90.0;

pub fn parse_point_string(point_string: &str) -> Result<Option<(f64, f64)>> {
    let mut coords = point_string.split(',');
    match (coords.next(), coords.next()) {
        (Some(latitude), Some(longitude)) => Ok(Some((latitude.parse::<f64>()?, longitude.parse::<f64>()?))),
        (_, _) => Ok(None),
    }
}

pub fn parse_points_string(points_string: &str) -> Result<Option<Vec<(f64, f64)>>> {
    let points: Vec<(f64, f64)> = points_string
        .split_whitespace()
        .filter_map(|point_string| parse_point_string(point_string).ok())
        .filter_map(|option| option)
        .collect();

    if points.len() > 0 {
        Ok(Some(points))
    } else {
        Ok(None)
    }
}

#[test]
fn test_parse_points_string() {
    let mut points = parse_points_string("48.0,-89.0 48,-89").unwrap().unwrap();
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

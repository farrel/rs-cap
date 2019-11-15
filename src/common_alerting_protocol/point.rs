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
    pub fn parse_point_string(point_string: &str) -> Self {
        let mut coords = point_string.split(',');
        let latitude = coords.nth(0).unwrap();
        let longitude = coords.nth(1).unwrap();

        return Point {
            latitude: latitude.parse::<f64>().unwrap(),
            longitude: longitude.parse::<f64>().unwrap(),
        };
    }

    pub fn parse_points_string(points_string: &str) -> Vec<Self> {
        return points_string
            .split(' ')
            .map(|coord_string| Point::parse_point_string(coord_string))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::point::Point;

    #[test]
    fn initialise() {
        let point = Point {
            latitude: 0.0,
            longitude: 0.0,
        };
        assert_eq!(0.0, point.latitude);
        assert_eq!(0.0, point.longitude);
    }
}

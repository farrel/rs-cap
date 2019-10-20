const MAX_LONGITUDE: f64 = 180.0;
const MIN_LONGITUDE: f64 = -180.0;

const MAX_LATITUDE: f64 = 90.0;
const MIN_LATITUDE: f64 = 90.0;

#[derive(Debug)]
pub struct Point {
    latitude:  f64,
    longitude: f64
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::point::Point;

    #[test]
    fn initialise() {
        let point = Point{ latitude: 0.0, longitude: 0.0};
        assert_eq!(0.0, point.latitude);
        assert_eq!(0.0, point.longitude);
    }
}

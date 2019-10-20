use crate::common_alerting_protocol::point::Point;

pub struct Circle {
    latitude:  f64,
    longitude: f64,
    radius:   f64
}


#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::circle::Circle;

    #[test]
    fn initialise() {
        let circle = Circle{ latitude: 0.0, longitude: 0.0, radius: 5.0};

        assert_eq!(0.0, circle.latitude);
        assert_eq!(0.0, circle.longitude);
        assert_eq!(5.0, circle.radius);
    }
}

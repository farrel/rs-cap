use crate::common_alerting_protocol::circle::Circle;
use crate::common_alerting_protocol::geocode::Geocode;
use crate::common_alerting_protocol::polygon::Polygon;

pub struct Area {
    area_desc: String,
    altitude:  f64,
    ceiling:   f64,
    circles:   Vec<Circle>,
    geocodes:  Vec<Geocode>,
    polygons:  Vec<Polygon>
}

mod point;
mod circle;
mod parameter;
mod geocode;

mod common_alerting_protocol {
    use chrono::prelude::*;
    use crate::common_alerting_protocol::point::Point;
    use crate::common_alerting_protocol::circle::Circle;
    use crate::common_alerting_protocol::parameter::Parameter;
    use crate::common_alerting_protocol::geocode::Geocode;

    struct EventCode {
        name:  String,
        value: String
    }

    struct Resource {
        resource_desc: String,
        mime_type:     String,
        size:          u64,
        uri:           String,
        digest:        String
    }

    struct Polygon {
        points: Vec<Point>
    }

    struct Area {
        area_desc: String,
        altitude:  f64,
        ceiling:   f64,
        circles:   Vec<Circle>,
        geocodes:  Vec<Geocode>,
        polygons:  Vec<Polygon>
    }

    struct Info {
        event:       String,
        urgency:     String,
        severity:    String,
        certainty:   String,
        language:    String,
        audience:    String,
        effective:   DateTime<Utc>,
        onset:       DateTime<Utc>,
        expires:     DateTime<Utc>,
        sender_name: String,
        headlnie:    String,
        description: String,
        instruction: String,
        web:         String,
        contact:     String
    }

    struct Alert {
        identifier:  String,
        sender:      String,
        sent:        DateTime<Utc>,
        status:      String,
        msg_type:    String,
        scope:       String,
        source:      String,
        restriction: String,
        notes:       String,
        addresses:   Vec<String>,
        codes:       Vec<String>,
        references:  Vec<String>,
        incidents:   Vec<String>,
        infos:       Vec<Info>
    }
}

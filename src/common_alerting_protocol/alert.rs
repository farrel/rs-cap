use chrono::prelude::*;
use crate::common_alerting_protocol::info::Info;

pub struct Alert {
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

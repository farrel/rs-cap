use crate::common_alerting_protocol::utilities::*;
use crate::common_alerting_protocol::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    pub sender: String,
    pub identifier: String,
    pub sent: DateTime<FixedOffset>,
}

impl Reference {
    pub(crate) fn parse_reference_string(refererences_str: &str) -> Result<Vec<Reference>> {
        let references: Vec<Reference> = Vec::new();

        Ok(split_string(refererences_str)?
            .iter()
            .filter_map(|reference_str| Reference::parse_string(reference_str).ok())
            .collect())
    }
    pub(crate) fn parse_string(reference_str: &str) -> Result<Reference> {
        let reference_vec: Vec<&str> = reference_str.split(",").collect();
        let sender = String::from(*reference_vec.get(0).expect("No reference"));
        let identifier = String::from(*reference_vec.get(1).expect("No identifier"));
        let sent = chrono::DateTime::parse_from_rfc3339(*reference_vec.get(2).expect("No Sent"))?;

        Ok(Reference { sender, identifier, sent })
    }
}

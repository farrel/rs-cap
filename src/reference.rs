use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::result::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    pub sender: String,
    pub identifier: String,
    pub sent: DateTime<FixedOffset>,
}

impl Reference {
    pub(crate) fn parse_string(reference_str: &str) -> Result<Reference> {
        let reference_vec: Vec<&str> = reference_str.split(',').collect();
        if reference_vec.len() == 3 {
            let sender = String::from(reference_vec[0]);
            let identifier = String::from(reference_vec[1]);
            match chrono::DateTime::parse_from_rfc3339(reference_vec[2]) {
                Ok(sent) => Ok(Reference { sender, identifier, sent }),
                Err(_) => Err(Error::ParseReference(String::from(reference_str))),
            }
        } else {
            Err(Error::ParseReference(String::from(reference_str)))
        }
    }
}

fn test_parse_reference_error(reference_string: &str) {
    match Reference::parse_string(reference_string) {
        Ok(_) => assert!(false, "Expected an error"),
        Err(error) => match error {
            Error::ParseReference(string) => assert_eq!(String::from(reference_string), string),
            error => assert!(false, "Expected a Error::ParseReference, got {:?}", error),
        },
    }
}

#[test]
fn test_parse_reference() {
    assert!(Reference::parse_string("kundtjanst@smhi.se,2.49.0.0.752.0.SE.230129125019.2956.26828,2023-01-29T12:50:19+00:00").is_ok());
    test_parse_reference_error("kundtjanst@smhi.se,2.49.0.0.752.0.SE.230129125019.2956.26828,202301291250190000");
    test_parse_reference_error("kundtjanst@smhi.se2.49.0.0.752.0.SE.230129125019.2956.268282023-01-29T12:50:19+00:00");
    test_parse_reference_error(",,");
}

// use quick_xml::Reader;
// use rs_cap::common_alerting_protocol::alert::{Alert, Version};
// use std::fs;
//
// #[test]
// fn deserialise_xml_file() {
//     let xml = fs::read_to_string("tests/alaska.cap").expect("Something went wrong reading the file");
//     let reader = &mut Reader::from_str(&xml);
//     reader.trim_text(true);
//
//     println!("{}", "test");
//
//     let alert = Alert::deserialize_from_xml(reader).unwrap();
//     assert_eq!(Some(Version::V1_2), alert.version);
//     assert_eq!(alert.identifier.unwrap(), "urn:oid:2.49.0.1.124.1576205950.2019");
// }

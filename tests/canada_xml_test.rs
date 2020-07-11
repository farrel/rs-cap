use quick_xml::Reader;
use rs_cap::common_alerting_protocol::alert;
use std::fs;

#[test]
fn deserialise_xml_file() {
    let xml = fs::read_to_string("tests/canada.xml").expect("Something went wrong reading the file");
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let reader = &mut Reader::from_str(&xml);
    reader.trim_text(true);

    let alert = alert::Alert::deserialize_from_xml(alert::VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();
    assert_eq!("urn:oid:2.49.0.1.124.1576205950.2019", alert.identifier.unwrap());
    assert_eq!("cap-pac@canada.ca", alert.sender.unwrap());
    assert_eq!(Some(alert::Status::Actual), alert.status);
    assert_eq!(Some(alert::MsgType::Update), alert.msg_type);
    assert_eq!("Env. Can. - Can. Met. Ctr. – Montréal", alert.source.unwrap());
    assert_eq!(Some(alert::Scope::Public), alert.scope);
    assert_eq!(6, alert.codes.len());
    assert!(alert.note.is_some());
    assert!(alert.sent.is_some());
    assert_eq!(3, alert.references.len());
    assert_eq!(2, alert.infos.len());
    assert!(alert.restriction.is_none());
}

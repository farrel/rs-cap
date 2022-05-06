use crate::point::parse_points_string;
use crate::result::Result;
use crate::utilities::read_string;
use geo::{LineString, Polygon};
use quick_xml::Reader;

pub const POLYGON_TAG: &[u8] = b"polygon";

pub fn deserialize_from_xml(
    namespace: &[u8],
    reader: &mut Reader<&[u8]>,
    buf: &mut std::vec::Vec<u8>,
    ns_buf: &mut std::vec::Vec<u8>,
) -> Result<Option<Polygon<f64>>> {
    match read_string(namespace, reader, buf, ns_buf, POLYGON_TAG)? {
        Some(points_string) => match parse_points_string(&points_string)? {
            Some(coords) => Ok(Some(Polygon::new(LineString::from(coords), vec![]))),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

#[test]
fn test_deserialise_from_xml() {
    use crate::alert::VERSION_1_2;
    use quick_xml::Reader;

    let xml = r#"<polygon xmlns="urn:oasis:names:tc:emergency:cap:1.2">48.5448,-89.0388 48.5001,-89.0231 48.4482,-89.0071 48.3079,-89.1102
        48.3096,-89.1933 48.3065,-89.2028 48.3101,-89.2143 48.3119,-89.271
        48.334,-89.3597 48.3528,-89.3957 48.3838,-89.4298 48.4198,-89.4449
        48.484,-89.432 48.519,-89.4061 48.551,-89.3458 48.59,-89.1847
        48.5859,-89.1228 48.568,-89.0639 48.5448,-89.0388
      </polygon>"#;

    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let reader = &mut Reader::from_str(xml);
    reader.trim_text(true);
    reader.read_namespaced_event(&mut buf, &mut ns_buf).unwrap();

    let polygon = deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap().unwrap();
    assert_eq!(19, polygon.exterior().num_coords());
}

#[test]
fn test_deserialise_from_namespaced_xml() {
    use crate::alert::VERSION_1_2;
    use quick_xml::Reader;

    let xml = r#"<cap:polygon xmlns:cap="urn:oasis:names:tc:emergency:cap:1.2">-27.77,-64.50 -27.86,-64.06 -28.47,-63.38 -28.86,-63.85 -28.14,-64.57 -27.77,-64.50</cap:polygon>"#;

    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let reader = &mut Reader::from_str(xml);
    reader.trim_text(true);
    reader.read_namespaced_event(&mut buf, &mut ns_buf).unwrap();

    let polygon = deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap().unwrap();
    assert_eq!(6, polygon.exterior().num_coords());
}

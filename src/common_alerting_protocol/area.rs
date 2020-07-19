use crate::common_alerting_protocol::circle::{Circle, CIRCLE_TAG};
use crate::common_alerting_protocol::deserialise_error::DeserialiseError;
use crate::common_alerting_protocol::geocode::{Geocode, GEOCODE_TAG};
use crate::common_alerting_protocol::polygon::{Polygon, POLYGON_TAG};
use crate::common_alerting_protocol::utilities::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

pub struct Area {
    area_desc: String,
    altitude: Option<f64>,
    ceiling: Option<f64>,
    circles: Vec<Circle>,
    geocodes: Vec<Geocode>,
    polygons: Vec<Polygon>,
}

pub const AREA_TAG: &[u8] = b"area";

const AREA_DESC_TAG: &[u8] = b"areaDesc";
const ALTITUDE_TAG: &[u8] = b"altitude";
const CEILING_TAG: &[u8] = b"ceiling";

impl Area {
    pub fn deserialize_from_xml(
        namespace: &[u8],
        reader: &mut Reader<&[u8]>,
        buf: &mut std::vec::Vec<u8>,
        ns_buf: &mut std::vec::Vec<u8>,
    ) -> Result<Area, DeserialiseError> {
        let mut area = Area {
            area_desc: String::new(),
            altitude: None,
            ceiling: None,
            circles: Vec::new(),
            polygons: Vec::new(),
            geocodes: Vec::new(),
        };

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(ref e)) if ns == namespace => match e.local_name() {
                    AREA_DESC_TAG => area.area_desc.push_str(read_string(namespace, reader, buf, ns_buf, AREA_DESC_TAG)?.as_str()),
                    POLYGON_TAG => area.polygons.push(Polygon::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    GEOCODE_TAG => area.geocodes.push(Geocode::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    ALTITUDE_TAG => area.altitude = Some(read_string(namespace, reader, buf, ns_buf, ALTITUDE_TAG)?.parse::<f64>()?),
                    CEILING_TAG => area.ceiling = Some(read_string(namespace, reader, buf, ns_buf, CEILING_TAG)?.parse::<f64>()?),
                    CIRCLE_TAG => area.circles.push(Circle::deserialize_from_xml(namespace, reader, buf, ns_buf)?),
                    unknown_tag => return Err(DeserialiseError::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },
                (Some(ns), Event::End(ref e)) if ns == namespace => match e.local_name() {
                    AREA_TAG => return Ok(area),
                    unknown_tag => return Err(DeserialiseError::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::alert::VERSION_1_2;
    use crate::common_alerting_protocol::area::Area;
    use quick_xml::Reader;

    #[test]
    fn deserialize_from_xml() {
        let xml = r#"<area xmlns="urn:oasis:names:tc:emergency:cap:1.2">
      <areaDesc>City of Thunder Bay</areaDesc>
      <altitude>100</altitude>
      <ceiling>200</ceiling>
      <polygon>
        48.5448,-89.0388 48.5001,-89.0231 48.4482,-89.0071 48.3079,-89.1102
        48.3096,-89.1933 48.3065,-89.2028 48.3101,-89.2143 48.3119,-89.271
        48.334,-89.3597 48.3528,-89.3957 48.3838,-89.4298 48.4198,-89.4449
        48.484,-89.432 48.519,-89.4061 48.551,-89.3458 48.59,-89.1847
        48.5859,-89.1228 48.568,-89.0639 48.5448,-89.0388
      </polygon>
      <geocode>
        <valueName>layer:EC-MSC-SMC:1.0:CLC</valueName>
        <value>048100</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558003</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558004</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558011</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558028</value>
      </geocode>
      <geocode>
        <valueName>profile:CAP-CP:Location:0.3</valueName>
        <value>3558090</value>
      </geocode>
    </area>"#;

        let _reader = &mut Reader::from_str(xml);
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        let reader = &mut Reader::from_str(xml);
        reader.trim_text(true);
        reader.read_namespaced_event(&mut buf, &mut ns_buf);

        let area = Area::deserialize_from_xml(VERSION_1_2.as_bytes(), reader, &mut buf, &mut ns_buf).unwrap();

        assert_eq!("City of Thunder Bay", area.area_desc);
        assert_eq!(1, area.polygons.len());
        let point = &area.polygons[0].points[0];
        assert_eq!(48.5448, point.latitude.unwrap());
        assert_eq!(-89.0388, point.longitude.unwrap());
        assert_eq!(6, area.geocodes.len());
        assert_eq!(Some(100.0), area.altitude);
        assert_eq!(Some(200.0), area.ceiling);
    }
}

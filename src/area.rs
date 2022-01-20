use crate::circle::{Circle, CIRCLE_TAG};
use crate::geocode::{Geocode, GEOCODE_TAG};
use crate::utilities::read_string;
use crate::{polygon, Error, Result};
use geo::Polygon;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Area {
    pub area_desc: Option<String>,
    pub altitude: Option<f64>,
    pub ceiling: Option<f64>,
    pub circles: Vec<Circle>,
    pub geocodes: Vec<Geocode>,
    pub polygons: Vec<Polygon<f64>>,
}

pub const AREA_TAG: &[u8] = b"area";

const AREA_DESC_TAG: &[u8] = b"areaDesc";
const ALTITUDE_TAG: &[u8] = b"altitude";
const CEILING_TAG: &[u8] = b"ceiling";
const POLYGON_TAG: &[u8] = b"polygon";

impl Area {
    pub fn initialise() -> Area {
        Area {
            area_desc: None,
            altitude: None,
            ceiling: None,
            circles: Vec::new(),
            geocodes: Vec::new(),
            polygons: Vec::new(),
        }
    }
    pub fn deserialize_from_xml(namespace: &[u8], reader: &mut Reader<&[u8]>, buf: &mut std::vec::Vec<u8>, ns_buf: &mut std::vec::Vec<u8>) -> Result<Area> {
        let mut area = Area::initialise();

        loop {
            match reader.read_namespaced_event(buf, ns_buf)? {
                (Some(ns), Event::Start(ref e)) if ns == namespace => match e.local_name() {
                    AREA_DESC_TAG => area.area_desc = read_string(namespace, reader, buf, ns_buf, AREA_DESC_TAG)?,
                    POLYGON_TAG => {
                        if let Some(polygon) = polygon::deserialize_from_xml(namespace, reader, buf, ns_buf)? {
                            area.polygons.push(polygon)
                        }
                    }
                    GEOCODE_TAG => {
                        if let Some(geocode) = Geocode::deserialize_from_xml(namespace, reader, buf, ns_buf)? {
                            area.geocodes.push(geocode)
                        }
                    }
                    ALTITUDE_TAG => area.altitude = read_string(namespace, reader, buf, ns_buf, ALTITUDE_TAG)?.and_then(|string| string.parse::<f64>().ok()),
                    CEILING_TAG => area.ceiling = read_string(namespace, reader, buf, ns_buf, CEILING_TAG)?.and_then(|string| string.parse::<f64>().ok()),
                    CIRCLE_TAG => {
                        if let Some(circle) = Circle::deserialize_from_xml(namespace, reader, buf, ns_buf)? {
                            area.circles.push(circle)
                        }
                    }
                    unknown_tag => return Err(Error::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },
                (Some(ns), Event::End(ref e)) if ns == namespace => match e.local_name() {
                    AREA_TAG => return Ok(area),
                    unknown_tag => return Err(Error::tag_not_expected(str::from_utf8(unknown_tag)?)),
                },
                _ => (),
            }
        }
    }

    pub fn add_circle<F>(&mut self, build_circle: F)
    where
        F: Fn(&mut Circle),
    {
        let mut circle = Circle::initialise();
        build_circle(&mut circle);
        self.circles.push(circle);
    }

    pub fn add_geocode<F>(&mut self, build_geocode: F)
    where
        F: Fn(&mut Geocode),
    {
        let mut geocode = Geocode::initialise();
        build_geocode(&mut geocode);
        self.geocodes.push(geocode);
    }
}

#[cfg(test)]
mod tests {
    use crate::alert::VERSION_1_2;
    use crate::area::Area;
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

        assert_eq!(Some(String::from("City of Thunder Bay")), area.area_desc);
    }
}

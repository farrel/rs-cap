use rs_cap::common_alerting_protocol::alert;
use std::fs;

#[test]
fn deserialize_cap_files() {
    for entry in fs::read_dir("tests/cap_files").unwrap() {
        let path = entry.unwrap().path();
        if !path.is_dir() {
            let xml_string = fs::read_to_string(&path).expect("Something went wrong reading the file");
            println!("{:?}", path);
            alert::parse(&xml_string).expect(&format!("Could not deserialize {:?}", path));
        }
    }
}

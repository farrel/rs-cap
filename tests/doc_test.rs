use quick_xml::events::Event;
use quick_xml::Reader;
use std::str::from_utf8;

#[test]
fn doc_test() {
    let xml = r#"<x:tag1 xmlns:x="www.xxxx" xmlns:y="www.yyyy" att1 = "test">
                <y:tag2><!--Test comment-->Test</y:tag2>
                <y:tag2>Test 2</y:tag2>
            </x:tag1>"#;
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut count = 0;
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    let mut txt = Vec::new();
    loop {
        match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
            Ok((ref ns, Event::Start(ref e))) => {
                count += 1;
                match (*ns, e.local_name()) {
                    (Some(b"www.xxxx"), b"tag1") => println!("xxx tag1"),
                    (Some(b"www.yyyy"), b"tag2") => println!("yyy tag2"),
                    (ns, n) => panic!("Namespace and local name mismatch"),
                }
                println!("Resolved namespace: {:?}", ns.and_then(|ns| from_utf8(ns).ok()));
            }
            Ok((_, Event::Text(e))) => txt.push(e.unescape_and_decode(&reader).expect("Error!")),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok((_, Event::Eof)) => break,
            _ => (),
        }
        buf.clear();
    }
    println!("Found {} start events", count);
    println!("Text events: {:?}", txt);
}

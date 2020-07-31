use rs_cap::common_alerting_protocol::alert::*;
use rs_cap::common_alerting_protocol::info::*;

#[test]
fn test_builder() {
    let mut alert = Alert::initialise();

    alert.add_info(|info| {
        info.audience = Some(String::from("Test"));
        info.certainty = Some(rs_cap::common_alerting_protocol::info::Certainty::Observed);

        info.add_event_code(|event_code| {
            event_code.name = Some(String::from("Name1"));
            event_code.value = Some(String::from("Value1"));
        });

        info.add_parameter(|parameter| {
            parameter.name = Some(String::from("Name2"));
            parameter.value = Some(String::from("Value2"));
        });

        info.categories.push(Category::Geological);

        info.add_resource(|resource| {
            resource.resource_desc = Some(String::from("resource_desc"));
            resource.size = Some(256);
        });
    });

    let info = alert.infos.last().unwrap();

    assert_eq!(Some(rs_cap::common_alerting_protocol::info::Certainty::Observed), info.certainty);
    assert_eq!(1, info.event_codes.len());
    assert_eq!(1, info.parameters.len());
    assert_eq!(1, info.categories.len());
    assert_eq!(1, info.resources.len());
}

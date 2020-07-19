use rs_cap::common_alerting_protocol::alert::*;


#[test]
fn test_builder() {
    let mut alert = Alert::initialise();

    let info = alert.add_info(|info| {
        info.audience = Some(String::from("Test"));
        info.certainty = Some(rs_cap::common_alerting_protocol::info::Certainty::Observed);
    });

    assert_eq!(Some(String::from("Test")), info.audience);
    assert_eq!(Some(rs_cap::common_alerting_protocol::info::Certainty::Observed), info.certainty);
}

pub struct Parameter{
    name:   String,
    value: String
}


#[cfg(test)]
mod tests {
    use crate::common_alerting_protocol::parameter::Parameter;

    #[test]
    fn initialise() {
        let parameter = Parameter{ name: String::from("name"), value: String::from("value") };

        assert_eq!(String::from("name"), parameter.name);
        assert_eq!(String::from("value"), parameter.value);
    }
}

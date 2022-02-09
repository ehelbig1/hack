use chrono::NaiveDateTime;
use serde::Deserialize;

pub type Crts = Vec<Crt>;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Crt {
    pub issuer_ca_id: u32,
    pub issuer_name: String,
    pub common_name: String,
    pub name_value: String,
    pub id: u64,
    pub entry_timestamp: NaiveDateTime,
    pub not_before: NaiveDateTime,
    pub not_after: NaiveDateTime,
    pub serial_number: String,
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use chrono::NaiveDate;
    use serde_json;

    #[test]
    fn should_deserialize() {
        let json = "[{\"issuer_ca_id\":1,\"issuer_name\":\"C=US, O=Let's Encrypt, CN=R3\",\"common_name\":\"test.example.com\",\"name_value\":\"test.example.com\",\"id\":1,\"entry_timestamp\":\"2022-02-03T10:11:13.479\",\"not_before\":\"2022-02-03T09:11:13\",\"not_after\":\"2022-05-04T09:11:12\",\"serial_number\":\"1\"}]";

        let expect: Crts = vec![Crt {
            issuer_ca_id: 1,
            issuer_name: String::from("C=US, O=Let's Encrypt, CN=R3"),
            common_name: String::from("test.example.com"),
            name_value: String::from("test.example.com"),
            id: 1,
            entry_timestamp: NaiveDate::from_ymd(2022, 2, 3).and_hms_milli(10, 11, 13, 479),
            not_before: NaiveDate::from_ymd(2022, 2, 3).and_hms(9, 11, 13),
            not_after: NaiveDate::from_ymd(2022, 5, 4).and_hms(9, 11, 12),
            serial_number: String::from("1"),
        }];

        let got: Crts = serde_json::from_str(json).unwrap();

        assert_eq!(got, expect)
    }
}

use serde::Deserialize;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person<'a> {
    id: i32,
    name: &'a str,
    localized_name: &'a [u8],
}

#[cfg(test)]
mod test {
    use crate::Person;

    #[test]
    fn deserialize_using_serde_json() {
        let json = r#"{"id":14,"name":"Elvis Wang","localizedName":"王先生"}"#;

        let (encoded, _, error) = encoding_rs::GB18030.encode(json);
        assert!(!error, "Encoded error");

        let obj: Person = serde_json::from_slice(encoded.as_ref()).expect("Failed deserialize");
        println!("Deserialized {:?}", obj);
    }

    #[test]
    fn deserialize_using_sonic_rs() {
        let json = r#"{"id":14,"name":"Elvis Wang","localizedName":"王先生"}"#;

        let (encoded, _, error) = encoding_rs::GB18030.encode(json);
        assert!(!error, "Encoded error");

        let obj: Person = sonic_rs::from_slice(encoded.as_ref()).expect("Failed deserialize");
        println!("Deserialized {:?}", obj);
    }
}

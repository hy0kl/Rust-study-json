extern crate rustc_serialize;
use std::collections::BTreeMap;
use rustc_serialize::json::{self, Json, ToJson};

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    code: u8,
    message: String,
    data: Vec<String>,
}

// Specify encoding method manually
impl ToJson for TestStruct {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        // All standard types implement `to_json()`, so use it
        d.insert("code".to_string(), self.code.to_json());
        d.insert("message".to_string(), self.message.to_json());
        d.insert("data".to_string(), self.data.to_json());
        Json::Object(d)
    }
}

fn main() {
    let object = TestStruct {
        code: 0,
        message: "Rues API is OK.".to_string(),
        data: vec![
            "192.168.0.11".to_string(),
            "192.168.2.19".to_string(),
        ],
    };

    // Serialize using `json::encode`
    let encoded = json::encode(&object).unwrap();
    println!("{}", encoded);

    // Deserialize using `json::decode`
    let decoded: TestStruct = json::decode(&encoded).unwrap();
    println!("JSON->data.length() = {:?}", decoded.data.len());

    let json_obj: Json   = object.to_json();
    let json_str: String = json_obj.to_string();
    println!("json: {}", json_str);
}

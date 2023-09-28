use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::blocking::Client;

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseObject {
    name: String,
    id: i32,
    color: String,
    language: String,
}

pub fn insert_hardcoded() -> Result<String, Box<dyn Error>> {
    // Base parts of connection string
    let rest_base = "http://localhost:8080/v1";
    let kvs_endpoint = format!("{}/kvs", rest_base);

    // Components of the Key
    let namespace = "test";
    let setname = "users";
    let userkey = "bob";

    let record_uri = format!("{}/{}/{}/{}", kvs_endpoint, namespace, setname, userkey);

    let request_body = r#"{
        "name": "James",
        "id": 123,
        "color": "Blue,
        "language": "Ingrish"
    }"#;

    let client = Client::new();

    let response = client.post(record_uri).body(request_body).send()?;

    println!("{:?}", response);

    Ok("it werked.".to_string())
}

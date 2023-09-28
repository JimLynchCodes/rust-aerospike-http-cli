use serde_json::Value;
use std::error::Error;

use reqwest::blocking::Client;

struct DatabaseObject {
    name: String,
    id: i32,
    color: String,
    language: String,
}

struct PostBody {
    json: DatabaseObject,
}

pub fn insert_hardcoded() -> Result<Value, Box<dyn Error>> {
    // Base parts of connection string
    let rest_base = "http://localhost:8080/v1";
    let kvs_endpoint = format!("{}/kvs", rest_base);

    // Components of the Key
    let namespace = "test";
    let setname = "users";
    let userkey = "bob";

    let record_uri = format!("{}/{}/{}/{}", kvs_endpoint, namespace, setname, userkey);

    // The content to be stored into Aerospike.
    let obj = DatabaseObject {
        name: "Bob".to_string(),
        id: 123,
        color: "Blue".to_string(),
        language: "Ingrish".to_string(),
    };

    let post_body = PostBody { json: obj };

    let client = reqwest::Client::new();

    let response = client.post(record_uri).body(post_body).send();

    let serde_result = serde_json::from_str::<Value>(&response)?;

    Ok(serde_result)
}

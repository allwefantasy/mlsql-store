use rocket::response::content;
use rocket::get;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HelloRes {
    name: String,
    age: u8,
}

#[get("/<name>/<age>")]
pub fn get_hello(name: String, age: u8) -> content::Json<String> {
    let res = HelloRes { name, age };

    content::Json(serde_json::to_string(&res).unwrap())
}
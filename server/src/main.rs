#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{
    self,
    get,
    post,
    routes,
    request::{
        Form,
    },
    config::{
        Config,
        Environment
    }
};

use rocket_contrib::serve::StaticFiles;

pub use shared::Data;

#[get("/api/data")]
fn fetch_data() -> String {
    let data = Data {
        val: 8,
        text: "Hello from Server".to_string(),
    };

    serde_json::to_string(&data).unwrap()
}

#[post("/api/data", data = "<data>")]
fn post_data(data: Form<Data>) -> String {
    if data.val > 7 {
        let response = Data {
            val: data.val,
            text: "Greater than 7".to_string(),
        };
        serde_json::to_string(&response).unwrap()
    } else {
        if data.text.contains("ping") {
            let response = Data {
                val: data.val,
                text: "pong".to_string(),
            };
            serde_json::to_string(&response).unwrap()
        } else {
            let response = Data {
                val: data.val,
                text: "No Comment".to_string(),
            };
            serde_json::to_string(&response).unwrap()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(3000) //TODO: read from args (gumdrop?)
        .finalize()?;

    rocket::custom(config)
        .mount("/", StaticFiles::from("public/"))
        .mount("/", routes![fetch_data, post_data])
        .launch();

    Ok(())
}

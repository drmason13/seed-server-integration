#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{
    self,
    get,
    routes,
    config::{
        Config,
        Environment
    }
};

use rocket_contrib::serve::StaticFiles;

pub use shared::Data;

#[get("/data")]
fn fetch_data() -> String {
    let data = Data {
        val: 8,
        text: "Hello from Server".to_string(),
    };

    serde_json::to_string(&data).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(3000)
        .finalize()?;

    rocket::custom(config)
        .mount("/", StaticFiles::from("../client"))
        .mount("/", routes![fetch_data])
        .launch();

    Ok(())
}

#![feature(proc_macro_hygiene, decl_macro)]
use anyhow::{anyhow, Context, Result};
use rocket::{
    self,
    config::{Config, Environment},
    get, post,
    routes,
};
use rocket_contrib::{serve::StaticFiles, json::Json};

use shared::users::{self, fields::*};

fn make_token(pwd: &Password) -> Result<Token> {
    if pwd.len() < 8 {
        Err(anyhow!("Password too short"))
    } else {
        Ok(Token("123456".to_string()))
    }
}

#[post("/users", data = "<new_user>")]
fn create_user(new_user: Json<users::create::Request>)
    -> Result<Json<users::create::Response>>
{
    let new_user = new_user.clone();
    let token = make_token(&new_user.password)
        .context("Failed to create token from Password")?;

    let response = users::create::Response {
        username: new_user.username,
        email: new_user.email,
        token,
        bio: Bio(String::new()),
        image: Image(String::new()),
    };

    Ok(Json(response))
}

fn main() -> Result<()> {
    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(3000) //TODO: read from args (gumdrop?)
        .finalize()?;

    rocket::custom(config)
        .mount("/", StaticFiles::from("public/"))
        .mount("/api", routes![
            create_user,
        ])
        .launch();

    Ok(())
}

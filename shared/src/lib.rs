use serde::{Deserialize, Serialize};

#[cfg(feature = "rocket")]
use rocket::request::FromForm;

#[cfg_attr(feature = "rocket", derive(FromForm))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub val: i8,
    pub text: String,
}

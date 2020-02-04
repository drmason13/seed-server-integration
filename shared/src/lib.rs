use serde::{Serialize, Deserialize};

#[cfg(feature = "rocket")]
use rocket::request::{FromForm};

#[cfg_attr(feature = "rocket", derive(FromForm))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub val: i8,
    pub text: String,
}

// all the newtypes proved a hassle so just using primitives for now
// trait Validate<'a>: Sized {
//     fn validate(unvalidated: &'a str) -> Result<Self, &'a str>;
// }
// 
// impl<'a> Validate<'a> for Val {
//     fn validate(unvalidated: &'a str) -> Result<Self, &'a str> {
//         match unvalidated.parse::<i8>() {
//             Ok(valid) => Ok(valid),
//             _ => Err(unvalidated),
//         }
//     }
// }
// 
// #[cfg(feature = "rocket")]
// impl<'v> FromFormValue<'v> for Val {
//     type Error = &'v RawStr;
// 
//     fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
//         Self::validate(form_value.as_str()).map_err(RawStr::from_str)
//     }
// }
// 
// #[cfg(feature = "rocket")]
// impl<'v> FromFormValue<'v> for Text {
//     type Error = &'v RawStr;
// 
//     fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
//         match form_value.len() {
//             len if len < 100 => Ok(form_value.to_string()),
//             _ => Err(form_value),
//         }
//     }
// }

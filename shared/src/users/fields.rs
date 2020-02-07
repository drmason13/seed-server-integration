use serde::{Serialize, Deserialize};
use shrinkwraprs::Shrinkwrap;

#[derive(Clone, Debug, Shrinkwrap, Serialize, Deserialize)]
pub struct Username(pub String);

#[derive(Clone, Debug, Shrinkwrap, Serialize, Deserialize)]
pub struct Email(pub String);

#[derive(Clone, Debug, Shrinkwrap, Serialize, Deserialize)]
pub struct Password(pub String);

#[derive(Clone, Debug, Shrinkwrap, Serialize, Deserialize)]
pub struct Token(pub String);

#[derive(Clone, Debug, Shrinkwrap, Serialize, Deserialize)]
pub struct Bio(pub String);

#[derive(Clone, Debug, Shrinkwrap, Serialize, Deserialize)]
pub struct Image(pub String);

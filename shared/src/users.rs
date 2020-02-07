use serde::{Serialize, Deserialize};

pub const URL: &'static str = "/api/users";

pub mod fields;
use fields::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub username: Username,
    pub email: Email,
    pub token: Token,
    pub bio: Bio,
    pub image: Image,
}

pub mod create {
    use serde::{Serialize, Deserialize};
    use super::fields::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
       pub username: Username,
       pub email: Email,
       pub password: Password,
    } 
    
    pub use super::Response;
}

pub mod read {
    use serde::{Serialize, Deserialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request;
    
    pub use super::Response;
}

pub mod update {
    use serde::{Serialize, Deserialize};
    use super::fields::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub username: Username,
        pub email: Email,
        pub token: Token,
        pub bio: Bio,
        pub image: Image,
    }
    
    pub use super::Response;
}
    
pub mod login {
    use serde::{Serialize, Deserialize};
    use super::fields::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub email: Email,
        pub password: Password,
    }
    
    pub use super::Response;
}

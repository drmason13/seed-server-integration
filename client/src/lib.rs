use seed::{prelude::*, *};

pub use shared::user::{self, fields::*};

type ServerResponse<T> = fetch::ResponseDataResult<T>;

// ---
//
// Model
//
// ---

#[derive(Default)]
pub struct Model {}

#[derive(Clone)]
enum Msg {
    Fetched(ServerResponse<user::create::Response>),
    Test,
}

// ---
//
// Update
//
// ---

async fn post_data() -> Result<Msg, Msg> {
    Request::new(shared::user::URL)
        .method(Method::Post)
        .send_json(&shared::user::create::Request {
            username: Username("Dave".to_string()),
            email: Email("TODO: Validation".to_string()),
            password: Password("12345678".to_string()),
        })
        .fetch_json_data(Msg::Fetched)
        .await
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetched(Ok(data)) => {
            log!(data);
            orders.skip();
        }
        Msg::Fetched(Err(err)) => {
            error!(format!("Fetch error: {:?}", err));
            orders.skip();
        }
        Msg::Test => {
            orders.perform_cmd(post_data());
        }
    }
}

// ---
//
// View
//
// ---

fn view(model: &Model) -> Node<Msg> {
    main![
        button![simple_ev(Ev::Click, Msg::Test), "Register Dummy User (temp)"],
    ]
}

#[wasm_bindgen(start)]
pub fn run() {
    App::builder(update, view).build_and_start();
}

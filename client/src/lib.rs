use seed::{*, prelude::*};

pub use shared;

// ---
//
// Model
//
// ---

#[derive(Default)]
pub struct Model {
    pub data: Option<shared::Data>,
}

#[derive(Clone)]
enum Msg {
    FetchData,
    Fetched(fetch::ResponseDataResult<shared::Data>),
}

// ---
//
// Update
//
// ---

async fn fetch_data() -> Result<Msg, Msg> {
    let url = "/api/data";

    Request::new(url)
        .method(Method::Get)
        .fetch_json_data(Msg::Fetched)
        .await
}

async fn post_data() -> Result<Msg, Msg> {
    let url = "/api/data";

    Request::new(url)
        .method(Method::Post)
        .send_json(&shared::Data { val: 8, text: "server will error if I don't include text".to_string() })
        .fetch_json_data(Msg::Fetched)
        .await
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchData => {
            orders.skip().perform_cmd(fetch_data());
        },
        Msg::Fetched(Ok(data)) => {
            model.data = Some(data);
        },
        Msg::Fetched(Err(err)) => {
            model.data = None;
            error!(format!("Fetch error: {:?}", err));
            orders.skip();
        },
    }
}

// ---
//
// View
//
// ---

fn view(model: &Model) -> Node<Msg> {
    main![
        view_display_data(&model.data),
        button![simple_ev(Ev::Click, Msg::FetchData), "Fetch data"],
        view_post_data_form(),
    ]
}

fn view_display_data(data: &Option<shared::Data>) -> Node<Msg> {
    section![
        h3!["data fetched from server:"],
        match data {
            Some(data) => {
                let shared::Data { val, text } = data;
                p![
                    "Received a value of ",
                    span![
                        style!{
                            St::Color => if *val < 0 { "red" } else { "blue" };
                        },
                        val.to_string()
                    ],
                    " and the accompanying text:",
                    br![],
                    text,
                ]
            },
            None => {
                p![ "no data!" ]
            },
        },
    ]
}

fn view_post_data_form() -> Node<Msg> {
    section![
        form![
            attrs!{ At::Action => "/api/data", At::Method => "Post" },
            legend!["Update data stored in server:"],
            "value:", br![],
            input![ attrs!{ At::Type => "text", At::Name => "val", At::Placeholder => "value" } ], br![], br![],
            input![ attrs!{ At::Type => "text", At::Name => "text", At::Placeholder => "text" } ], br![], br![],
            input![ attrs!{ At::Type => "submit", At::Value => "Update" } ],
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn run() {
    App::builder(update, view)
        .build_and_start();
}

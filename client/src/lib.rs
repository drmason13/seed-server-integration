use seed::{prelude::*, *};

pub use shared::users::{self, fields::*};

type ServerResponse<T> = fetch::ResponseDataResult<T>;

// ---
//
// Model
//
// ---

#[derive(Debug)]
pub struct Model {
    // can the Api be used as a "form" directly?
    // Or do we need to write a `Form` for every Request that needs one?
    // simply hardcoded for now, still working on forms
    form: users::create::Request,
    user: Option<users::create::Response>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            form: users::create::Request {
                username: Username("Dave".to_string()),
                email: Email("TODO: Validation".to_string()),
                password: Password("12345678".to_string()),
            },
            user: None,
        }
    }
}

#[derive(Clone, Debug)]
enum Msg {
    UserCreateResponse(ServerResponse<users::create::Response>),
    FormSubmit,
    FormChange,
}

// ---
//
// Update
//
// ---

async fn send_user_create_request(user: shared::users::create::Request)
    -> Result<Msg, Msg>
{
    Request::new(shared::users::URL)
        .method(Method::Post)
        .send_json(&user)
        .fetch_json_data(Msg::UserCreateResponse)
        .await
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UserCreateResponse(Ok(data)) => {
            log!(data);
            model.user = Some(data);
        }
        Msg::UserCreateResponse(Err(err)) => {
            error!(format!("User creation error: {:?}", err));
            model.user = None;
            orders.skip();
        }
        Msg::FormSubmit => {
            orders.perform_cmd(
                send_user_create_request(model.form.clone())
            );
        }
        Msg::FormChange => unimplemented!()
    }
}

// ---
//
// View
//
// ---

fn view(model: &Model) -> Node<Msg> {
    main![
        h3!["Dummy Form"],
        form![
            input!["username", attrs!{
                At::Name => "username",
                At::Type => "text",
                At::Value => model.form.username.as_str(),
            }],
            input!["email", attrs!{
                At::Name => "email",
                At::Type => "text",
                At::Value => model.form.email.as_str(),
            }],
            input!["password", attrs!{
                At::Name => "password",
                At::Type => "password",
                At::Value => model.form.password.as_str(),
            }],
        ],
        button![simple_ev(Ev::Click, Msg::FormSubmit), "Register Dummy User"],
        if let Some(ref user) = model.user {
            section![
                h3!["Registered user"],
                p![format!("username: `{:?}`", user.username)],
                p![format!("token: `{:?}`", user.token)],
                p![format!("bio: `{:?}`", user.bio)],
            ]
        } else { empty![] }
    ]
}

#[wasm_bindgen(start)]
pub fn run() {
    App::builder(update, view).build_and_start();
}

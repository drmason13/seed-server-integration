# rust web app: seed frontend and rocket backend

## Overview

### Workspace

This is a cargo [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) with 3 member crates:

* `client`
* `server`
* `shared`

The workspace was made by simply creating a new directory and adding a `Cargo.toml` with the following content:

Cargo.toml

```
[workspace]

members = [
    "client",
    "server",
    "shared",
]
```

### client

`client` stores the frontend code that compiles to wasm. The crate used is
[seed](https://crates.io/crates/seed), though a notable alternative is [yew](https://crates.io/crates/yew).

The client is a cargo lib and depends on `shared` for the definitions of shared
data types, in this toy example:, just the `Data` struct.

The client was created using `cargo new --lib client` and editing Cargo.toml to read:

client/Cargo.toml

```
[package]
name = "client"
version = "0.1.0"
authors = ["masond <drmason13@gmail.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
seed = "0.5.1"
wasm-bindgen = "0.2.58"

shared = { path = "../shared" }
```

`shared = { path = "../shared" }` points to our "shared types crate" locally, it's not published on crates.io.
More on this crate later.

### server

`server` is a rocket application that will serve two purposes:

1.  To serve the static assets of the web app itself (html, css and js)
2.  To process requests received from the web app (a REST API)

It uses the [rocket](https://crates.io/crates/rocket) crate to do this, which requires the nightly compiler. An alternative that runs on stable rust is [Actix-web](https://crates.io/crates/actix-web).

The server is a binary package and is made using `cargo new server` and editing the Cargo.toml:

server/Cargo.toml

```
[package]
name = "server"
version = "0.1.0"
authors = ["masond <drmason13@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rocket = "0.4.2"
rocket_contrib = "0.4.2"
serde = { version = "1.0.104", features = ['derive'] }
serde_json = "1.0.45"

shared = { path = "../shared" }
anyhow = "1.0.26"
shrinkwraprs = "0.3.0"
```

### shared

shared is a library of types for use by both client and server. This
example defines a small subset of the realworld (conduit) API relating
to users.

shared/src/users.rs

```
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

// ...
```

The module structure is modelled after the API, using CRUD\* for structure.

It derives serde's traits for sending/receiving to/from the
client/server as JSON. The exact rust types survive this process thanks
to [serde](https://crates.io/crates/serde).

## Build

### client and wasm-pack

Building the client requries wasm-pack `cargo install wasm-pack` and the following incantation:

```
wasm-pack build client --target web --out-name package --dev
```

The name is used in this index.html file in the server crate in public/:

server/public/index.html

```
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">

    <meta name="description" content="">

    <link rel="icon" type="image/png" href="/public/favicon.png">

    <title>Seed and Rocket server Integration Example</title>

    </head>

    <body>
        <section id="app"></section>
        <script type="module">
            // https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
            import init from '/pkg/package.js';
            init('/pkg/package_bg.wasm');
        </script>

    </body>
</html>
```

`import init from pkg/package.js` is the bind code that hooks in our web
assembly frontend code, _package_ is the `--out-name` passed to wasm-pack. It's
arbitrary but we must be consistent.

The `pkg/` folder is created by wasm-pack and contains our wasm code, so we
need it in server/public/ next to our index.html or rocket won't be able to
serve it.

### rocket server and cargo-make

When building rocket we can simply use cargo as it is a normal rust binary:
`cargo build --package server`

`--package server` is telling cargo which member of the workspace to build - no
point building the client to a non-wasm target!

We also want to make sure that the client is built and that the output (`pkg/`)
is ready to serve in `/public`.

While we could manage this manually in the terminal or maybe write a shell
script, let's lean on a tool designed for this: [cargo-make](https://sagiegurari.github.io/cargo-make/).

The following `Makefile.toml` will be understood by cargo-make and allow us to run:

```
cargo make start
```

To build everything, copy the wasm files to where we need them and start the server.

Makefile.toml

```
# run `cargo make start` to build everything and start the server

[tasks.start]
description = "Build and start server, serving the client as static assets"
workspace = false
cwd = "server"
command = "cargo"
args = ["run"]
dependencies = [
    "build",
    "copy_wasm",
]

[tasks.build]
clear = true
workspace = false
description = "Build client and server"
run_task = { name = [ "build_server", "build_client" ] }

[tasks.build_server]
description = "Build server"
workspace = false
command = "cargo"
args = ["build", "--package", "server"]

[tasks.build_client]
description = "Build client"
workspace = false
install_crate = { crate_name = "wasm-pack", binary = "wasm-pack", test_arg = "-V" }
command = "wasm-pack"
args = ["build", "client", "--target", "web", "--out-name", "package", "--dev"]

[tasks.copy_wasm]
description = "place wasm-pack output in server public/ folder"
command = "cp"
args = [
    "-r",
    "client/pkg/",
    "server/public/",
]
```

Note: You will need to install cargo-make first, it's another [cargo subcommand](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands)

## Sending requests

Add a variant to your `Msg` that can store a response [seed has its own types for this]()

```
		// we have a type alias wrapping Seed's type
		type ServerResponse<T> = fetch::ResponseDataResult<T>;

		// ...

enum Msg {
    UserCreateResponse(ServerResponse<users::create::Response>),
		// ...
}
```

Add a function to do the sending, it can return a `Result`.
It should use the type for the request imported from `shared`.

```
async fn send_user_create_request(user: shared::users::create::Request)
    -> Result<Msg, Msg>
{
    Request::new(shared::users::URL)
        .method(Method::Post)
        .send_json(&user)
        .fetch_json_data(Msg::UserCreateResponse)
        .await
}
```

Seed's `fetch_json_data()` assumes that we are uninterested in the full server
response (status, headers as well as data) and just passes the data to a Msg.
Very convenient, and if you do want to check the server response, there's
`fetch_json()`

Add Msg Handlers (match arms) to the update function:

1.  `Msg::FormSubmit` triggers the sending of the data
2.  `Msg::UserCreateResponse(Ok(data)) handles the happy path of getting the data ok`
3.  `Msg::UserCreateResponse(Err(err)) handles the unhappy path of an error from the server`

```
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
    }
}
```

Of course you will want a way to trigger the fetching of the data. One option is a button in the view somewhere:

```
button![simple_ev(Ev::Click, Msg::FormSubmit), "Register User"],
```

Other options are on page load, which I remember reading is possible, but the exact details escape me right now.

## Responding to requests

Rocket makes this easy :)

TL;DR

```
#[post("/users", data = "<new_user>")]
fn create_user(new_user: Json<users::create::Request>)
    -> Result<Json<users::create::Response>>
    // ... do stuff and create response
    Ok(Json(response))
}
```

The exact type of `response` (`users::create::Response`) arrives inside
the Msg in client. The types are in fact identical since they are both
imported from `shared`.

## Tests

This example doesn't include tests, and I removed the dependency on
`wasm-bindgen-test` to avoid distraction. However, a real project will need
tests and I'd like to add the setup for them to this repo once I know
how they work.

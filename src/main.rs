use std::collections::HashMap;
use std::env;
use std::num::ParseIntError;
use std::sync::Mutex;

use actix_web::{server, App, HttpRequest, HttpResponse, Json};
use sentry::integrations::failure::capture_error as capture_failure_error;
use sentry::protocol::value::to_value;
use sentry::User;
use sentry_actix::SentryMiddleware;
use serde::{Deserialize, Serialize};
use actix_web::middleware::cors::Cors;
use actix_web::{http::StatusCode};


lazy_static::lazy_static! {
    static ref HASHMAP: Mutex<HashMap<&'static str, u32>> = {
        let mut inventory = HashMap::new();
        inventory.insert("wrench", 1);
        inventory.insert("nails", 1);
        inventory.insert("hammer", 1);
        Mutex::new(inventory)
    };
}

fn multiply_new(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number: i32 = first_number_str.parse()?;
    let second_number: i32 = second_number_str.parse()?;
    Ok(first_number * second_number)
}

fn handled(_: ()) -> String {
    let first = "t";
    let second = "2";
    match multiply_new(first, second) {
        Ok(result) => format!("{} * {} => {}", first, second, result),
        Err(err) => {
            sentry::capture_error(&err);
            "try again".to_string()
        }
    }
}

fn fakedatabseapp(_req: &HttpRequest) -> HttpResponse {
    panic!("Unhandled request!");
}

#[derive(Deserialize, Clone, Debug)]
struct CardSubmittedPayload {
    card_id: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Item {
    id: String,
    name: String,
    price: f64,
    img: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
struct CheckoutPayload {
    email: String,
    cart: Vec<Item>,
}



fn process_order(cart: &[Item]) -> HttpResponse  {
    let mut map = HASHMAP.lock().unwrap();


    for cartitem in cart.iter() {
        match map.get_mut(cartitem.id.as_str()) {
            Some(stock) if *stock > 0 => {
                *stock -= 1;
                println!(
                    "Success: {} was purchased, remaining stock is {}",
                    cartitem.id, stock
                );
            }
            _ => {
                sentry::configure_scope(|scope| {
                    scope.set_extra("inventory", to_value(&*map).unwrap());
                });


                let message = format!("Not enough inventory for {}", cartitem.id);
                println!("The entry for `0` is \"{:?}\".", message);
                capture_failure_error(&failure::format_err!("Error: {}", message));
                return HttpResponse::InternalServerError().content_type("text/plain").body("external service error");
            }
        }
    }

    return HttpResponse::Ok().content_type("text/plain").body("Ok");
}

fn checkout(req: HttpRequest, body: Json<CheckoutPayload>) -> HttpResponse {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(User {
            email: Some(body.email.clone()),
            ..User::default()
        }));

        let headers = req.headers();

        let transaction_id = headers.get("X-Transaction-ID").unwrap().to_str().unwrap();
        scope.set_tag("transaction_id", transaction_id);

        let session_id = headers.get("X-Session-ID").unwrap().to_str().unwrap();
        scope.set_tag("session_id", session_id);
    });

    process_order(&body.cart)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let _guard =
        sentry::init("https://ef73d8aa7ac643d2b6f1d1e604d607eb@o87286.ingest.sentry.io/5250920");

        server::new(|| {
            App::new()
                .middleware(SentryMiddleware::new())
                .configure(|app| {
                    Cors::for_app(app)
                        .allowed_origin("http://localhost:5000")
                        .allowed_methods(vec!["GET", "POST"])
                        .max_age(3600)
                        .resource("/handled", |r| {
                            r.get().with(handled)
                        })
                        .resource("/unhandled", |r| {
                            r.get().f(fakedatabseapp)
                        })
                        .resource("/checkout", |r| r.post().with(checkout))
                        .register()
                })
        })
        .bind("127.0.0.1:3001")
        .unwrap()
        .run();
}

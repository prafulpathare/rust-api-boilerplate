#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    email: String,
    password: String
}

#[post("/users", format = "json", data = "<user_input>")]
fn add_user(user_input: Json<User>) -> Json<User> {
    let user = User {
        id: user_input.id,
        email: user_input.email.to_string(),
        password: user_input.password.to_string()
    };
    Json(user)
}


#[get("/")]
fn index() -> String {
    format!("<h3>Welcome to Rust API</h3>")
}

fn main() {
    rocket::ignite().mount("/", routes![index, add_user]).launch();
}
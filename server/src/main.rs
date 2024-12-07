#![allow(unused)]
extern crate serde;
#[macro_use]
extern crate rocket;
use rocket::{Build, Rocket};
extern crate rocket_dyn_templates;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_firebase_auth::FirebaseAuth;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
extern crate kalosm;
use kalosm::language::*;

mod chat;
mod common;

#[rocket::launch]
async fn rocket() -> Rocket<Build> {
    #[derive(Parse, Clone)]
    pub enum Response {
        Data(String),
    }

    // Create a parser and wrap it in Arc
    let parser = Arc::new(Response::new_parser());

    // Initialize the Llama model and the chat session
    let model = Llama::new_chat().await.unwrap();
    let chat = Chat::builder(model)
        .with_constraints(move |_history| parser.clone())
        .with_system_prompt(
            "The assistant will act like a secretary. Respond with JSON in the format \
            { \"data\": \"hello\" } ",
        )
        .build();

    // Wrap the chat session in a shared state
    let chat_state = Arc::new(Mutex::new(chat));

    // Setup firebase authentication
    let firebase_auth: FirebaseAuth = FirebaseAuth::builder()
        .json_file("src/firebase-credentials.json")
        .build()
        .expect("Failed to read firebase credentials");

    // Setup cors
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            ["Get", "Post", "Put", "Delete", "Options"]
                .iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to setup cors configuration.");

    rocket::build()
        .manage(chat_state)
        .manage(firebase_auth)
        .mount("/", chat::routes())
        .mount("/", rocket_cors::catch_all_options_routes())
        .attach(cors.clone())
        .manage(cors)
}

use rocket::{
    http::Status,
    routes,
    serde::json::Json,
    Route
};
use std::{ptr::null, sync::Arc};
use tokio::sync::Mutex;
use crate::common::{ApiResponse, Message, BotResponse, JsonResponse};
use kalosm::language::*;
use serde_json::from_str;
use rocket_firebase_auth::FirebaseToken;
use std::collections::HashMap;

pub fn routes() -> Vec<Route> {
    routes![chat]
}

// Approach 1
// #[post("/submit", format = "json", data = "<message>")]
// async fn chat(token: FirebaseToken, message: Json<Message>, chat_state: &rocket::State<Arc<Mutex<Chat>>>) 
// -> ApiResponse<BotResponse> {
//     // Access the chat session from the shared state
//     let mut chat: tokio::sync::MutexGuard<'_, Chat> = chat_state.lock().await;

//     // Add the user's message to the chat history and get response
//     let stream: ChannelTextStream = chat.add_message(&message.chat);

//     // Response
//     let ai_response: String = stream.collect::<Vec<String>>().await.join("");
//     let parsed: JsonResponse = from_str(&ai_response).expect("Failed to parse JSON");
//     let data_field: String = parsed.data;

//     // Return the response as JSON
//     ApiResponse {
//         json: Some(Json(BotResponse {
//             success: true,
//             message: data_field,
//         })),
//         status: Status::Ok,
//     }
// }

// Approach 2
#[post("/submit", format = "json", data = "<message>")]
async fn chat(token: FirebaseToken, message: Json<Message>, model_state: &rocket::State<Arc<Mutex<Llama>>>, chat_map_state: &rocket::State<Arc<Mutex<HashMap<String, Chat>>>>) 
-> ApiResponse<BotResponse> {
    #[derive(Parse, Clone)]
    pub enum Response {
        Data(String),
    }

    // Access the chat session from the shared state
    let mut model = model_state.lock().await;
    let mut chat_map: tokio::sync::MutexGuard<'_, HashMap<String, Chat>> = chat_map_state.lock().await;
    let mut chat: &mut Chat;

    // Get or create the chat
    let chat: &mut Chat = if let Some(chat_opt) = chat_map.get_mut(&token.sub) {
        // Use the existing chat
        chat_opt
    } else {
        // Create a parser and wrap it in Arc
        let parser = Arc::new(Response::new_parser());

        // Create a new chat
        // Cloning model might be slow, but I don't have a better approach yet
        let new_chat = Chat::builder(model.clone())
            .with_constraints(move |_history| parser.clone())
            .with_system_prompt(
                "The assistant will act like a secretary. Respond with JSON in the format \
                { \"data\": \"hello\" } ",
            )
            .build();

        // Insert the new chat into the map and then get a mutable reference
        chat_map.insert(token.sub.clone(), new_chat);
        chat_map.get_mut(&token.sub).unwrap()
    };

    // Add the user's message to the chat history and get response
    let stream: ChannelTextStream = chat.add_message(&message.chat);

    // Response
    let ai_response: String = stream.collect::<Vec<String>>().await.join("");
    let parsed: JsonResponse = from_str(&ai_response).expect("Failed to parse JSON");
    let data_field: String = parsed.data;

    // Return the response as JSON
    ApiResponse {
        json: Some(Json(BotResponse {
            success: true,
            message: data_field,
        })),
        status: Status::Ok,
    }
}
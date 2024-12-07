use rocket::{
    http::Status,
    routes,
    serde::json::Json,
    Route
};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::common::{ApiResponse, Message, BotResponse, JsonResponse};
use kalosm::language::*;
use serde_json::from_str;
use rocket_firebase_auth::FirebaseToken;

pub fn routes() -> Vec<Route> {
    routes![chat]
}

#[post("/submit", format = "json", data = "<message>")]
async fn chat(token: FirebaseToken, message: Json<Message>, chat_state: &rocket::State<Arc<Mutex<Chat>>>) 
-> ApiResponse<BotResponse> {
    // Access the chat session from the shared state
    let mut chat: tokio::sync::MutexGuard<'_, Chat> = chat_state.lock().await;

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
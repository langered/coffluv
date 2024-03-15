use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Coffee {
    brand: String,
    name: String,
    grind_size: u8,
    grind_time: f32,
    rating: Option<u8>,
}

pub async fn get_coffees() -> impl IntoResponse {
    let coffees = vec![
        Coffee {
            brand: "Agata".to_string(),
            name: "House Blend Espresso".to_string(),
            grind_size: 8,
            grind_time: 6.5,
            rating: Some(4),
        },
        Coffee {
            brand: "Agata".to_string(),
            name: "Fernando Escobar".to_string(),
            grind_size: 8,
            grind_time: 7.5,
            rating: Some(3),
        },
        Coffee {
            brand: "Agata".to_string(),
            name: "Henrique & Ricardo Barbosa".to_string(),
            grind_size: 8,
            grind_time: 6.5,
            rating: Some(5),
        },
        Coffee {
            brand: "Agata".to_string(),
            name: "Finca Rio Colorado".to_string(),
            grind_size: 7,
            grind_time: 6.5,
            rating: None,
        },
    ];

    (StatusCode::OK, Json(coffees))
}

pub async fn create_coffee(user_input: Json<Coffee>) -> impl IntoResponse {
    (StatusCode::CREATED, user_input)
}

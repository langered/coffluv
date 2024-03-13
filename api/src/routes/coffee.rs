use rocket::Route;
use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};

#[derive(Deserialize, Serialize)]
struct Coffee {
    name: String,
}

#[rocket::get("/coffees")]
fn coffee() -> Json<Vec<Coffee>> {
    let coffees = vec![
        Coffee {
            name: "Americano".to_string(),
        },
        Coffee {
            name: "Latte".to_string(),
        },
        Coffee {
            name: "Cappuccino".to_string(),
        },
    ];

    Json(coffees)
}

pub fn routes() -> Vec<Route> {
    routes![coffee]
}

// src/controllers/bank_controller.rs
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/banks")]
pub async fn get_banks() -> impl Responder {
    // Implement the logic to fetch the list of banks from the database
    // and map the data to the JSON structure you provided
    let banks_data = json!({
        "105": {
            "name": "AGROINDUSTRIJSKO KOMERCIJALNA BANKA AIK BANKA, AD, BEOGRAD",
            "img": ""
        },
        "115": {
            "name": "MOBI BANKA, AD, BEOGRAD",
            "img": ""
        },
        // TODO Add other bank entries in a similar manner
    });

    HttpResponse::Ok().json(banks_data)
}

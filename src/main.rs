use chrono::{DateTime, Utc};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::{fs, ops::Range};

fn print_time(prefix: &str) -> DateTime<Utc> {
    let now = Utc::now();
    println!("{prefix}: {} {}", now.date(), now.time());
    now
}

async fn test_rust() {
    let start_time = print_time("RUST START");

    let path = "./src/test_data.json";
    let data = fs::read_to_string(path).expect("Unable to read file!");
    let mut json_payload: Value = serde_json::from_str(&data).expect("Failed to parse JSON!");

    let client = reqwest::Client::new();

    for num in (Range { start: 0, end: 20 }) {
        json_payload["invoice_number"] = json!(num.to_string());

        let response = client
            .post("http://0.0.0.0:8001/invoice")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .json(&json_payload)
            .send()
            .await
            .unwrap();

        println!(
            "{num} Response: {:?}",
            response.headers().get("date").unwrap()
        );
    }

    let end_time = print_time("RUST END");
    println!(
        "\nTime taken: {} seconds",
        (end_time - start_time).num_milliseconds() as f64 / 1000.0
    )
}

async fn test_node() {
    let start_time = print_time("NODE START");

    let client = reqwest::Client::new();

    client
        .post("http://0.0.0.0:5000/api/actions")
        .header(AUTHORIZATION, "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjQyMjIsImF1dGhTdHJpbmciOiJhO2tsc2RqZjtrcGx3ZXFycml1cG90MnF3LXVwOWlvdHJmamlwW293cWVuO21rZGxmZ3ZhO2xpa2pwaHUgc2siLCJpYXQiOjE2MTU4NDM1MjB9.35QdvO_SGp5femVaTK-S9jsIikPD8WSc4JObxME-c5E")
        .json(&json!({ "action": "testPdfGeneration" }))
        .send()
        .await
        .unwrap();

    let end_time = print_time("NODE END");
    println!(
        "\nTime taken: {} seconds",
        (end_time - start_time).num_milliseconds() as f64 / 1000.0
    )
}

#[tokio::main]
async fn main() {
    test_rust().await;

    println!("\n===================================\n");

    test_node().await;
}

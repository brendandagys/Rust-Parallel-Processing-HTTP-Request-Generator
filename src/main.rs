use chrono::{DateTime, Utc};
use reqwest::{
    blocking::Client,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    fs,
    ops::Range,
    thread::{self, JoinHandle},
};

fn print_time(prefix: &str) -> DateTime<Utc> {
    let now = Utc::now();
    println!("{prefix}: {} {}", now.date(), now.time());
    now
}

async fn test_rust() {
    let start_time = print_time("\nRUST START");

    let path = "./src/test_data.json";
    let data = fs::read_to_string(path).expect("Unable to read file!");

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for num in (Range::<i32> { start: 0, end: 20 }) {
        let mut json_payload: Value = serde_json::from_str(&data).expect("Failed to parse JSON!");

        let join_handle = thread::spawn(move || {
            json_payload["invoice_number"] = json!(num.to_string());

            let client = Client::new();

            let response = client
                .post("http://0.0.0.0:8001/invoice")
                .header(CONTENT_TYPE, "application/json")
                .header(ACCEPT, "application/json")
                .json(&json_payload)
                .send()
                .expect("Request failed!");

            println!(
                "Thread {:0>2} response: {:?}",
                num,
                response.text().unwrap() // response.headers().get("date").unwrap()
            )
        });

        handles.push(join_handle)
    }

    for handle in handles {
        handle.join().unwrap()
    }

    let end_time = print_time("RUST END");
    println!(
        "\nRUST TIME TAKEN: {} seconds",
        (end_time - start_time).num_milliseconds() as f64 / 1000.0
    )
}

async fn test_node() {
    #[derive(Deserialize)]
    struct Payload {
        success: bool,
        invoicePdfs: Vec<String>,
        timeTaken: f64,
    }

    #[derive(Deserialize)]
    struct Data {
        actionResponse: Payload,
    }

    #[derive(Deserialize)]
    struct NodeResponse {
        data: Data,
    }

    let client = reqwest::Client::new();

    let response = client
        .post("http://0.0.0.0:5000/api/actions")
        .json(&json!({ "action": "testPdfGeneration" }))
        .send()
        .await
        .unwrap();

    let response_json = response.json::<NodeResponse>().await.unwrap();

    println!(
        "NODE TIME TAKEN: {} seconds",
        response_json.data.actionResponse.timeTaken
    )
}

#[tokio::main]
async fn main() {
    test_rust().await;
    test_node().await;
}

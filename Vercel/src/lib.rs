use std::collections::HashMap;

use anyhow::Result;
use libsql_client::DatabaseClient;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub env: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[wasm_bindgen]
pub async fn entry_point(input: JsValue) -> Result<JsValue, JsError> {
    let req: WasmRequest = serde_wasm_bindgen::from_value(input)?;

    let db = libsql_client::reqwest::Client::new(
        &req.env["LIBSQL_CLIENT_URL".into()],
        &req.env["LIBSQL_CLIENT_TOKEN".into()],
    );

    let response = db
        .execute("UPDATE counter SET count = count + 1 WHERE id = 1 RETURNING count")
        .await
        .expect("deez errz");

    let wasm_res = WasmResponse {
        status: 200,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: response
            .rows
            .get(0)
            .unwrap()
            .values
            .get(0)
            .unwrap()
            .to_string()
            .into_bytes(),
    };

    /*
    let wasm_res = WasmResponse {
        status: 200,
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("dsa".into(), "cxz".into()),
        ],
        body: serde_json::to_vec(&req).unwrap(),
    };
    */

    return Ok(serde_wasm_bindgen::to_value(&wasm_res)?);
}

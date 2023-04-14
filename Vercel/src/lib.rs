use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
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

    let wasm_res = WasmResponse {
        status: 200,
        headers: vec![("Content-Type".to_string(), "application/json".to_string()), ("dsa".into(), "cxz".into())],
        body: serde_json::to_vec(&req).unwrap(),
    };

    return Ok(serde_wasm_bindgen::to_value(&wasm_res)?);
}


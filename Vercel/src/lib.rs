use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl WasmResponse {
    pub fn new(status: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        WasmResponse {
            status,
            headers,
            body,
        }
    }

    pub fn empty() -> Self {
        WasmResponse {
            status: 200,
            headers: HashMap::new(),
            body: vec![],
        }
    }

    pub fn ok(content: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        WasmResponse {
            status: 200,
            headers,
            body: content.as_bytes().to_vec(),
        }
    }

    pub fn with_header(self, key: &str, value: &str) -> Self {
        let mut headers = self.headers;
        if let Some(h) = headers.get_mut(key) {
            *h = value.to_string();
        } else {
            headers.insert(key.to_string(), value.to_string());
        }

        WasmResponse {
            status: self.status,
            headers,
            body: self.body,
        }
    }

    pub fn with_headers(self, headers: HashMap<String, String>) -> Self {
        WasmResponse {
            status: self.status,
            headers,
            body: self.body,
        }
    }

    pub fn with_status(self, status: u16) -> Self {
        WasmResponse {
            status,
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn with_body(self, body: Vec<u8>) -> Self {
        WasmResponse {
            status: self.status,
            headers: self.headers,
            body,
        }
    }

    // etc...
}

#[wasm_bindgen]
pub async fn entry_point(input: JsValue) -> Result<JsValue, JsError> {
    let req: WasmRequest = serde_wasm_bindgen::from_value(input)?;
    let resp = route(req).await;
    return match resp {
        Ok(resp) => Ok(serde_wasm_bindgen::to_value(&resp)?),
        Err(err) => {
            // if dev here
            let resp = WasmResponse::new(
                500,
                HashMap::new(),
                format!("Error: {:?}", err).as_bytes().to_vec(),
            );

            Ok(serde_wasm_bindgen::to_value(&resp)?)
        }
    };
}

pub async fn route(req: WasmRequest) -> Result<WasmResponse> {
    // here logic to route to different handlers
    // it should be in different file because it will be generated using build.rs
    //
    let wasm_res = WasmResponse::ok("");
    let number: u32 = "321f".parse()?;

    return Ok(wasm_res);
}

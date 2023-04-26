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
    headers: Vec<(String, String)>,
    //         ^---- In future, this should be a HashMap
    body: Vec<u8>,
}

impl WasmResponse {
    pub fn new(status: u16, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        WasmResponse {
            status,
            headers,
            body,
        }
    }

    pub fn empty() -> Self {
        WasmResponse {
            status: 200,
            headers: vec![],
            body: vec![],
        }
    }

    pub fn ok(content: &str) -> Self {
        WasmResponse {
            status: 200,
            headers: vec![],
            body: content.as_bytes().to_vec(),
        }
    }

    pub fn with_header(self, key: &str, value: &str) -> Self {
        let mut headers = self.headers;
        headers.push((key.to_string(), value.to_string()));
        WasmResponse {
            status: self.status,
            headers,
            body: self.body,
        }
    }

    pub fn with_headers(self, headers: Vec<(String, String)>) -> Self {
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

    let wasm_res = WasmResponse {
        status: 200,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: vec![],
    };

    return Ok(serde_wasm_bindgen::to_value(&wasm_res)?);
}

pub async fn route(req: WasmRequest) -> Result<WasmResponse> {
    // here logic to route to different handlers
    // it should be in different file because it will be generated using build.rs
    //
    let wasm_res = WasmResponse {
        status: 200,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: vec![],
    };

    return Ok(wasm_res);
}

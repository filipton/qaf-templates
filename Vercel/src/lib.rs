use anyhow::Result;
use std::collections::HashMap;
use structs::{WasmRequest, WasmResponse};
use wasm_bindgen::prelude::*;
use web_sys::console;

mod router;
mod structs;

#[wasm_bindgen]
pub async fn entry_point(input: JsValue) -> Result<JsValue, JsError> {
    let req: WasmRequest = serde_wasm_bindgen::from_value(input)?;
    let resp = router::route(req).await;
    return match resp {
        Ok(resp) => Ok(serde_wasm_bindgen::to_value(&resp)?),
        Err(err) => {
            // if dev here
            console::log_1(&err.to_string().into());
            let resp = WasmResponse::new(
                500,
                HashMap::new(),
                format!("Error: {:?}", err).as_bytes().to_vec(),
            );

            Ok(serde_wasm_bindgen::to_value(&resp)?)
        }
    };
}

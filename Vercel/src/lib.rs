use anyhow::Result;
use qaf_router::{WasmRequest, WasmResponse};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod router;

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

//[[IF DATABASE Planetscale]]
pub fn get_ps_conn(req: &WasmRequest) -> Result<planetscale_driver::PSConnection> {
    let conn = planetscale_driver::PSConnection::new(
        req.env.get("PS_HOST").unwrap(),
        req.env.get("PS_USER").unwrap(),
        req.env.get("PS_PASS").unwrap(),
    );
    Ok(conn)
}
//[[ENDIF]]

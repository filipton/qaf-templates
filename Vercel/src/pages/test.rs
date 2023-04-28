use crate::structs::{WasmRequest, WasmResponse};
use anyhow::Result;
use qaf_macros::get;

#[get("/")]
pub async fn test(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok("test");
    Ok(res)
}

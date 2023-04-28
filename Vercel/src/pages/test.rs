use anyhow::Result;
use qaf_macros::{get, post};
use qaf_router::{WasmRequest, WasmResponse};

#[get("")]
pub async fn test(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok("test");
    Ok(res)
}

#[get("test")]
pub async fn test2(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok("test2");
    Ok(res)
}

#[post("test")]
pub async fn test_post(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok("test post");
    Ok(res)
}

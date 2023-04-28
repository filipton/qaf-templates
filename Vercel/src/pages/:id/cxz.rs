use anyhow::Result;
use qaf_macros::get;
use qaf_router::{WasmRequest, WasmResponse};

#[get("")]
pub async fn test_cxz(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok(&format!("{:?}", req.params));
    Ok(res)
}

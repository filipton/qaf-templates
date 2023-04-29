use anyhow::Result;
use qaf_macros::{get, post};
use qaf_router::{WasmRequest, WasmResponse};
//[[IF DATABASE Planetscale]]
use planetscale_driver::query;
//[[ENDIF]]

#[get("")]
pub async fn test(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok("test");
    Ok(res)
}

//[[IF DATABASE Planetscale]]
#[get("test")]
pub async fn test2(req: WasmRequest) -> Result<WasmResponse> {
    let mut conn = crate::get_ps_conn(&req)?;
    let val: u32 = query("SELECT 69420").fetch_scalar(&mut conn).await?;
    let res = WasmResponse::ok(&format!("Value: {}", val));
    Ok(res)
}
//[[ENDIF]]

#[post("test")]
pub async fn test_post(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok("test post");
    Ok(res)
}

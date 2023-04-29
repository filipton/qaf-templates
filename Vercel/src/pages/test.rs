use anyhow::Result;
use planetscale_driver::{query, PSConnection};
use qaf_macros::{get, post};
use qaf_router::{WasmRequest, WasmResponse};

#[get("")]
pub async fn test(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok("test");
    Ok(res)
}

#[get("test")]
pub async fn test2(req: WasmRequest) -> Result<WasmResponse> {
    let mut conn = PSConnection::new(
        req.env.get("PS_HOST").unwrap(),
        req.env.get("PS_USER").unwrap(),
        req.env.get("PS_PASS").unwrap(),
    );
    let val: u32 = query("SELECT 69420").fetch_scalar(&mut conn).await?;
    let res = WasmResponse::ok(&format!("Value: {}", val));
    Ok(res)
}

#[post("test")]
pub async fn test_post(req: WasmRequest) -> Result<WasmResponse> {
    let res = WasmResponse::ok("test post");
    Ok(res)
}

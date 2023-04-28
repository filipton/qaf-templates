//THIS FILE IS AUTOGENERATED, DO NOT EDIT
use crate::{WasmRequest, WasmResponse, WasmRouter};
use anyhow::Result;

#[path = "pages"]
pub mod pages {
    pub mod test;
}

pub async fn route(req: WasmRequest) -> Result<WasmResponse> {
    let router = WasmRouter::new()
        .get("/", pages::test::test)
        .get("/test", pages::test::test2)
        .post("/test", pages::test::test_post);

    let matched = router.routes.get(&req.method).unwrap().at(&req.url)?;
    let handler = matched.value;
    let resp = handler(req).await;

    return resp;
}

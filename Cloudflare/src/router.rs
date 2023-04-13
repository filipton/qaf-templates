//THIS FILE IS AUTOGENERATED, DO NOT EDIT
use worker::{Context, Env, Request, Response, Result, Router};

#[path = "pages"]
pub mod pages {
    pub mod tests;
    #[path = ":id"]
    pub mod _id {
        pub mod cxzcxz;
    }
    pub mod test {
        pub mod dsa;
    }
    pub mod nested {
        pub mod dsa;
    }
}

pub async fn router(req: Request, env: Env) -> Result<Response> {
    let router = Router::new();

    return router
        .get_async("/", pages::tests::index)
        .get_async("/fetch", pages::tests::fetch)
        .on_async("/ws", pages::tests::ws)
        .on_async("/:id", pages::_id::cxzcxz::route1)
        .get_async("/test/:id", pages::test::dsa::route1)
        .post_async("/test/:id/test", pages::test::dsa::route2)
        .on_async("/nested", pages::nested::dsa::hw_empty)
        .on_async("/nested/siem", pages::nested::dsa::siema)
        .run(req, env)
        .await;
}

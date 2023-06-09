//THIS FILE IS AUTOGENERATED, DO NOT EDIT
use actix_web::web;

#[path = "pages"]
pub mod pages {
    pub mod hello;
    pub mod nested {
        pub mod hello_there;
    }
}

pub fn generated_scope() -> actix_web::Scope {
    web::scope("")
        .service(pages::hello::get_test)
        .service(pages::hello::get_test_name)
        .service(web::scope("nested").service(pages::nested::hello_there::test))
}

use anyhow::Result;
use matchit::Router;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, future::Future, pin::Pin, rc::Rc};

/*
        ,----,
      ,/   .`|
    ,`   .'  :
  ;    ;     /              ,---,
.'___,/    ,'  ,---.      ,---.'|   ,---.
|    :     |  '   ,'\     |   | :  '   ,'\
;    |.';  ; /   /   |    |   | | /   /   |
`----'  |  |.   ; ,. :  ,--.__| |.   ; ,. :
    '   :  ;'   | |: : /   ,'   |'   | |: :
    |   |  ''   | .; :.   '  /  |'   | .; :
    '   :  ||   :    |'   ; |:  ||   :    |
    ;   |.'  \   \  / |   | '/  ' \   \  /
    '---'     `----'  |   :    :|  `----'
                       \   \  /
                        `----'

MOVE THIS FILE AWAY!!! MAYBE TO A SEPARATE CRATE
          OBVIOUSLY SPLITTED INTO MULTIPLE FILES
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,

    pub params: HashMap<String, String>,
    pub env: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl WasmResponse {
    #[allow(dead_code)]
    pub fn new(status: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        WasmResponse {
            status,
            headers,
            body,
        }
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        WasmResponse {
            status: 200,
            headers: HashMap::new(),
            body: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn ok(content: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        WasmResponse {
            status: 200,
            headers,
            body: content.as_bytes().to_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn not_found() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        WasmResponse {
            status: 404,
            headers,
            body: "Not Found".as_bytes().to_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    #[allow(dead_code)]
    pub fn add_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    #[allow(dead_code)]
    pub fn add_status(&mut self, status: u16) {
        self.status = status;
    }

    #[allow(dead_code)]
    pub fn with_header(self, key: &str, value: &str) -> Self {
        let mut headers = self.headers;
        headers.insert(key.to_string(), value.to_string());

        WasmResponse {
            status: self.status,
            headers,
            body: self.body,
        }
    }

    #[allow(dead_code)]
    pub fn with_headers(self, headers: HashMap<String, String>) -> Self {
        WasmResponse {
            status: self.status,
            headers,
            body: self.body,
        }
    }

    #[allow(dead_code)]
    pub fn with_status(self, status: u16) -> Self {
        WasmResponse {
            status,
            headers: self.headers,
            body: self.body,
        }
    }

    #[allow(dead_code)]
    pub fn with_body(self, body: Vec<u8>) -> Self {
        WasmResponse {
            status: self.status,
            headers: self.headers,
            body,
        }
    }

    // etc...
}

type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
type AsyncHandlerFn<'a> = Rc<dyn 'a + Fn(WasmRequest) -> LocalBoxFuture<'a, Result<WasmResponse>>>;

const METHODS: [&str; 9] = [
    "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "CONNECT", "PATCH", "TRACE",
];
pub struct WasmRouter<'a> {
    pub routes: HashMap<String, Router<AsyncHandlerFn<'a>>>,
}

impl<'a> WasmRouter<'a> {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        for method in METHODS.iter() {
            routes.insert(method.to_string(), Router::new());
        }
        Self { routes }
    }

    pub fn add_route<T>(&mut self, method: &str, path: &str, handler: fn(WasmRequest) -> T)
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        let handler: AsyncHandlerFn = Rc::new(move |req: WasmRequest| Box::pin(handler(req)));
        self.routes
            .get_mut(method)
            .unwrap()
            .insert(path, handler)
            .unwrap();
    }

    #[allow(dead_code)]
    pub fn get<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("GET", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn post<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("POST", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn put<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("PUT", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn delete<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("DELETE", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn head<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("HEAD", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn options<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("OPTIONS", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn connect<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("CONNECT", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn patch<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("PATCH", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn trace<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("TRACE", path, handler);
        self
    }
}

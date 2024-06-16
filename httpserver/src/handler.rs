use http::{
    httprequest::{HttpRequest, Resource},
    httpresponse::HttpResonse,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs};

pub trait Handler {
    fn handler(request: &HttpRequest) -> HttpResonse;

    /// 使用文件名从环境目录中读取文件。
    fn load_file(file_name: &str) -> Option<String> {
        // 获取编译时环境变量
        // See: https://rustwiki.org/zh-CN/core/macro.env.html
        // `CARGO_MANIFEST_DIR`: 包（crate）根目录
        // See: https://doc.rust-lang.org/cargo/reference/environment-variables.html
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

        // 获取运行时环境变量
        // See: https://rustwiki.org/zh-CN/std/env/fn.var.html
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

        let full_path = format!("{public_path}/{file_name}");

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

pub struct PageNotFoundHandler;

impl Handler for PageNotFoundHandler {
    fn handler(_request: &HttpRequest) -> HttpResonse {
        HttpResonse::new("404", None, Self::load_file("404.html"))
    }
}

pub struct StaticPageHandler;

impl Handler for StaticPageHandler {
    fn handler(request: &HttpRequest) -> HttpResonse {
        let Resource::Path(url) = &request.resource;
        let url = url.split('/').collect::<Vec<&str>>();

        let response = match url[1] {
            "" => HttpResonse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResonse::new("200", None, Self::load_file("health.html")),

            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut headers = HashMap::<&str, &str>::new();

                    if path.ends_with(".css") {
                        headers.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        headers.insert("Content-Type", "text/javascript");
                    } else {
                        headers.insert("Content-Type", "text/html");
                    }

                    HttpResonse::new("200", Some(headers), Some(contents))
                }
                None => HttpResonse::new("404", None, Self::load_file("404.html")),
            },
        };

        response
    }
}

pub struct WebServiceHandler;

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        // 获取编译时环境变量
        // See: https://rustwiki.org/zh-CN/core/macro.env.html
        // `CARGO_MANIFEST_DIR`: 包（crate）根目录
        // See: https://doc.rust-lang.org/cargo/reference/environment-variables.html
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));

        // 获取运行时环境变量
        // See: https://rustwiki.org/zh-CN/std/env/fn.var.html
        let public_path = env::var("DATA_PATH").unwrap_or(default_path);

        let full_path = format!("{public_path}/orders.json");

        let orders = fs::read_to_string(full_path);
        let orders = serde_json::from_str::<Vec<OrderStatus>>(orders.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handler(request: &HttpRequest) -> HttpResonse {
        let default_response = HttpResonse::new("404", None, Self::load_file("404.html"));

        let Resource::Path(url) = &request.resource;
        let url = url.split('/').collect::<Vec<&str>>();

        if url.len() < 4 {
            return default_response;
        }

        match url[2] {
            "shipping" if url[3] == "orders" => {
                let headers = HashMap::from([("Content-Type", "application/json")]);
                let body = serde_json::to_string(&Self::load_json()).unwrap();
                HttpResonse::new("200", Some(headers), Some(body))
            }

            _ => default_response,
        }
    }
}

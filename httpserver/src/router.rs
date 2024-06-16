use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::httprequest::{HttpRequest, Method, Resource};
use std::io::Write;

pub struct Router;

impl Router {
    /// 根据路由调用处理器。
    ///
    /// - GET 以外的方法以 Not Found 处理。
    /// - "api" 以外的路由以静态画面处理。
    ///
    /// # 参数
    ///
    /// - `request` - HTTP 请求([`HttpRequest`])。
    /// - `stream` - TCP 字节流。
    ///
    /// [`HttpRequest`]: http::httprequest::HttpRequest
    pub fn route(request: HttpRequest, stream: &mut impl Write) {
        let is_not_get = request.method != Method::Get;
        if is_not_get {
            PageNotFoundHandler::handler(&request).send_response(stream);
            return;
        }

        // `ref`: 引用绑定
        // See: https://rustwiki.org/zh-CN/std/keyword.ref.html
        let Resource::Path(ref url) = request.resource;
        let url = url.split('/').collect::<Vec<&str>>();

        let route = url[1];

        let is_not_api_route = route != "api";
        if is_not_api_route {
            StaticPageHandler::handler(&request).send_response(stream);
            return;
        }

        WebServiceHandler::handler(&request).send_response(stream);
    }
}

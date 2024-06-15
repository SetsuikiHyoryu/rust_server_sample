use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub message_body: String,
}

impl From<String> for HttpRequest {
    fn from(request: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path(String::new());
        let mut parsed_headers = HashMap::new();
        let mut parsed_message_body = String::new();

        for line in request.lines() {
            // 根据请求文中每行不同的部分判断这一行属于什么性质，
            // 并以不同的方式获取这行中的数据。
            if line.contains("HTTP") {
                // 请求行
                let (method, resource, version) = process_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(':') {
                // 请求头
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if !line.is_empty() {
                // 请求体（分割请求头和请求体的空行不需要处理）
                parsed_message_body = line.to_string();
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            message_body: parsed_message_body,
        }
    }
}

fn process_request_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();

    // `unwrap_or_default`: 如果前 None，返回该类型的默认值。
    // See: https://rustwiki.org/zh-CN/core/option/enum.Option.html#method.unwrap_or_default
    let method: Method = words.next().unwrap_or_default().into();
    let resource = Resource::Path(words.next().unwrap_or_default().to_string());
    let version = words.next().unwrap_or_default().into();

    (method, resource, version)
}

fn process_header_line(line: &str) -> (String, String) {
    let mut header_item = line.split(':');
    let key = header_item.next().unwrap_or_default().trim().to_string();
    let value = header_item.next().unwrap_or_default().trim().to_string();
    (key, value)
}

#[cfg(test)]
mod httprequest_tests {
    use super::*;

    #[test]
    fn test_method_into() {
        // `into()`：转换类型。
        // See: https://rustwiki.org/zh-CN/core/convert/trait.Into.html
        let method: Method = "GET".into();
        assert_eq!(method, Method::Get)
    }

    #[test]
    fn test_version_into() {
        // `into()`：转换类型。
        // See: https://rustwiki.org/zh-CN/core/convert/trait.Into.html
        let method: Version = "HTTP/1.1".into();
        assert_eq!(method, Version::V1_1)
    }

    #[test]
    fn test_read_http() {
        let request = String::from("GET /greeting HTTP/1.1\r\nHost: localhost\r\nAccept: */*\r\nUser-Agent: curl/7.71.1\r\n\r\nSample.");

        let mut headers_expected = HashMap::<String, String>::new();

        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), "curl/7.71.1".into());

        let request: HttpRequest = request.into();

        assert_eq!(Method::Get, request.method);
        assert_eq!(Version::V1_1, request.version);
        assert_eq!(Resource::Path("/greeting".to_string()), request.resource);
        assert_eq!(headers_expected, request.headers);
    }
}

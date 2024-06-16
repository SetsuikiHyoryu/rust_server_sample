use std::{collections::HashMap, fmt::Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResonse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

// See https://rustwiki.org/zh-CN/core/default/trait.Default.html
impl<'a> Default for HttpResonse<'a> {
    fn default() -> Self {
        HttpResonse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResonse<'a>> for String {
    fn from(response: HttpResonse) -> Self {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            response.version(),
            response.status_code(),
            response.status_text(),
            response.headers(),
            response.body().len(),
            response.body(),
        )
    }
}

impl<'a> HttpResonse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response = HttpResonse::default();

        if status_code != "200" {
            response.status_code = status_code;
        }

        response.headers = match headers {
            Some(..) => headers,
            None => {
                let map = HashMap::from([("Content-Type", "text/html")]);
                Some(map)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Unavaliable Request",
        };

        response.body = body;

        response
    }

    /// 发送响应。
    ///
    /// ## 参数
    ///
    /// - `writer` - 实现了 Write trait 的 TCP Stream。
    pub fn send_response(&self, writer: &mut impl Write) -> Result<(), Self> {
        let response = self.clone();
        let response = String::from(response);

        // 将字符串式响应发送给 TCP Stream
        let _ = write!(writer, "{}", response);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let headers = match &self.headers {
            Some(data) => data,
            None => {
                panic!("Fail to access headers.");
            }
        };

        let mut result = String::new();

        for (key, value) in headers.iter() {
            result = format!("{}{}: {}\r\n", result, key, value);
        }

        result
    }

    fn body(&self) -> &str {
        match &self.body {
            Some(data) => data,
            None => "",
        }
    }
}

#[cfg(test)]
mod httpresponse_tests {
    use super::*;

    #[test]
    fn test_response_creation_200() {
        let headers = Some(HashMap::from([("Content-Type", "text/html")]));
        let body = Some(String::new());

        let expected = HttpResonse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: headers.clone(),
            body: body.clone(),
        };

        let actual = HttpResonse::new("200", headers, body);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_response_creation_404() {
        let headers = Some(HashMap::from([("Content-Type", "text/html")]));
        let body = Some(String::new());

        let expected = HttpResonse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: headers.clone(),
            body: body.clone(),
        };

        let actual = HttpResonse::new("404", headers, body);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_response_converted_to_string() {
        let headers = Some(HashMap::from([("Content-Type", "text/html")]));
        let body = Some(String::new());

        let expected: String = HttpResonse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: headers.clone(),
            body: body.clone(),
        }
        .into();

        let actual =
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 0\r\n\r\n";

        assert_eq!(expected, actual);
    }
}

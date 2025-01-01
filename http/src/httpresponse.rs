use std::collections::HashMap;
use std::io::{Write};

// 结构体 HttpResponse
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}   

// 实现 Default
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(), 
            headers: None,
            body: None,
        }
    }
}

// 实现 From 
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
        let response = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &response.version(),
            &response.status_code(),
            &response.status_text(),
            &response.headers(),
            &response.body().len(),
            &response.body(),
        )
    }
    
}

// 实现 new 方法
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        
        // response  
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found", 
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, stream: &mut impl Write) -> std::io::Result<()> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = stream.write(response_string.as_bytes());
        Ok(())
    }

    //字段实现get方法 
    pub fn version(&self) -> &str {
        self.version
    }

    pub fn status_code(&self) -> &str {
        self.status_code
    }

    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut headers_string = String::new();
        for (k, v) in map.iter() {
            headers_string = format!("{}{}: {}\r\n", headers_string, k, v);
        }
        headers_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response = HttpResponse::new("200", None, Some("Hello, World!".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Hello, World!".into()),
        };
        
        assert_eq!(response, response_expected);
    }
    #[test]
    fn test_response_struct_creation_404() {
        let response = HttpResponse::new("404", None, Some("Hello, World!".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Hello, World!".into()),
        };
        
        assert_eq!(response, response_expected);
    }
    
    // 测试 from
    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Hello, World!".into()),
        };

        let http_string: String = response_expected.into();
        let actual_string = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, World!".to_string(); 
        
        assert_eq!(http_string, actual_string);
    }
}


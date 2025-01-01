use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}


//实现 From<&str> for Method
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }    
}


// 枚举 Version
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}


//实现 From<&str> for Version
impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }    
}

// 枚举 Resourse
#[derive(Debug, PartialEq)]
pub enum Resourse {
    Path(String),
}

pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resourse,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

// 实现 From<String> for HttpRequest
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resourse::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = String::new();
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line.to_string();
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
}

// 实现 process_req_line
fn process_req_line(s: &str) -> (Method, Resourse, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resourse::Path(resource.to_string()),
        version.into(),
    )
}

// 实现 process_header_line
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }
    (key, value)
} 





//测试单元

#[cfg(test)]
mod tests {
    use super::*;
    
    //测试 from<&str> for Method
    #[test]
    fn test_method_from_str() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
    }

    //测试 from<&str> for Version
    #[test]
    fn test_version_from_str() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    // 测试 test_read_http
    #[test]
    fn test_read_http() {
        let s = String::from("GET /index.html HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
        headers_expected.insert("Accept".into(), " */*".into());
        let req: HttpRequest = s.into();

        assert_eq!(req.method, Method::GET);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.resource, Resourse::Path("/index.html".to_string()));
        assert_eq!(req.headers, headers_expected);
    }
}


use std::collections::HashMap;
use std::io::{Result, Write};

//----------------------------------------------------------------
#[derive(Debug,PartialEq,Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

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

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
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
        let map : HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (key,value) in map.iter() {
            header_string = format!("{}{}:{}\n", header_string, key, value);
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => ""
        }
    }
}

//----------------------------------------------------------------
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\n{}Content-Length: {}\n\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }

}

//----------------------------------------------------------------
#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn test_reponse_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("Item was shipped on 20th July 2023".into()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text : "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 20th July 2023".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_reponse_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("Item was shipped on 20th July 2023".into()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text : "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 20th July 2023".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_reponse_struct_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 20th July 2023".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual = "HTTP/1.1 404 Not Found\nContent-Type:text/html\nContent-Length: 34\n\nItem was shipped on 20th July 2023";

        assert_eq!(response_actual, http_string);

    }
}
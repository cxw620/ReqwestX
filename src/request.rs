use http::{Extensions, HeaderMap, HeaderValue, Method, Response, Uri};

use crate::{
    error::ErrorType,
    ffi::{HttpHeaderFfi, HttpRequestFfi, ReqwestxGo, ReqwestxGoImpl},
};

/// Wrapper for HTTP request
pub struct Request {
    /// The request's method
    pub method: Method,

    /// The request's URI
    pub uri: Uri,

    /// The request's headers
    pub headers: HeaderMap<HeaderValue>,

    /// The request's extensions
    pub extensions: Extensions,

    /// limit body type to be Vec<u8> to match Go side
    pub body: Option<Vec<u8>>,
}

impl Request {
    #[inline]
    pub fn new(uri: Uri, method: Method) -> Self {
        Self {
            method,
            uri,
            headers: HeaderMap::with_capacity(32),
            extensions: Extensions::default(),
            body: None,
        }
    }

    #[inline]
    pub fn get(url: Uri) -> Self {
        Self::new(url, Method::GET)
    }

    #[inline]
    pub fn post(url: Uri) -> Self {
        Self::new(url, Method::POST)
    }

    #[inline]
    pub fn patch(url: Uri) -> Self {
        Self::new(url, Method::PATCH)
    }

    #[inline]
    pub fn delete(url: Uri) -> Self {
        Self::new(url, Method::DELETE)
    }

    #[inline]
    pub fn put(url: Uri) -> Self {
        Self::new(url, Method::PUT)
    }

    #[inline]
    pub fn head(url: Uri) -> Self {
        Self::new(url, Method::HEAD)
    }

    #[inline]
    pub fn options(url: Uri) -> Self {
        Self::new(url, Method::OPTIONS)
    }

    #[inline]
    pub fn set_method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    #[inline]
    pub fn set_body(mut self, body: Option<Vec<u8>>) -> Self {
        self.body = body;
        self
    }

    pub async fn execute(self) -> Result<Response<Vec<u8>>, ErrorType> {
        let method = match self.method {
            Method::GET => 0,
            Method::POST => 1,
            Method::PATCH => 2,
            Method::DELETE => 3,
            Method::PUT => 4,
            Method::HEAD => 5,
            Method::OPTIONS => 6,
            _ => return Err(ErrorType::UnsupportedHttpMethod),
        };

        let headers = self
            .headers
            .keys()
            .map(|k| HttpHeaderFfi {
                k: unsafe { String::from_utf8_unchecked(k.as_str().as_bytes().to_vec()) },
                v: self
                    .headers
                    .get_all(k)
                    .iter()
                    .map(|v| unsafe { String::from_utf8_unchecked(v.as_bytes().to_vec()) })
                    .collect(),
            })
            .collect();

        let req_ffi = HttpRequestFfi {
            url: self.uri.to_string(),
            method,
            body: self.body.unwrap_or_default(),
            headers,
        };

        ReqwestxGoImpl::send(req_ffi).await.into_result()
    }
}

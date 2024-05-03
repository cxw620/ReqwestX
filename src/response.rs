use http::{HeaderMap, HeaderName, HeaderValue, Response, Version};

use crate::ffi::HttpResponseFfi;

impl From<HttpResponseFfi> for Response<Vec<u8>> {
    fn from(value: HttpResponseFfi) -> Self {
        let version = match (value.proto_major, value.proto_minor) {
            (1, 0) => Version::HTTP_10,
            (1, 1) => Version::HTTP_11,
            (2, 0) => Version::HTTP_2,
            (3, 0) => Version::HTTP_3,
            _ => unreachable!("Invalid HTTP version"),
        };

        let mut response = Response::builder()
            .status(value.code as u16)
            .version(version)
            .body(value.data.clone()) // ? should we clone it here?
            .unwrap();

        let mut headers = HeaderMap::with_capacity(value.headers.len());
        value.headers.into_iter().for_each(|h| {
            // Safety: must be valid UTF-8 since Go has already validated it. Just unwrap it.
            // Can we ignore checking???
            let k = HeaderName::from_bytes(h.k.as_bytes()).unwrap();

            h.v.into_iter().for_each(|v| {
                // Safety: must be valid UTF-8 since Go has already validated it. Just copy it.
                let v = unsafe { HeaderValue::from_maybe_shared_unchecked(v) };
                // Really need to clone k since most of the time there's only one v.
                headers.append(k.clone(), v);
            });
        });
        let _ = std::mem::replace(response.headers_mut(), headers);

        response
    }
}

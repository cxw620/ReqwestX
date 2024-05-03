pub mod impersonate;
pub use crate::ffi::ClientConfig;

use crate::ffi::{ReqwestxGoInit, ReqwestxGoInitImpl};

/// Prepare the client, you should call this function before any other functions
#[inline]
pub fn init_client(config: ClientConfig) {
    ReqwestxGoInitImpl::init_client(config);
}

/// Set proxy
#[inline]
pub fn set_proxy(proxy: String) {
    ReqwestxGoInitImpl::set_proxy_ffi(proxy);
}

#[cfg(test)]
mod test {
    use crate::request::Request;

    use super::*;

    #[tokio::test]
    async fn test_client() {
        let config = ClientConfig {
            proxy: "socks5://127.0.0.1:9923".to_string(),
            impersonation_template: 0,
        };
        init_client(config);

        let uri = "https://tls.peet.ws/api/all".parse().unwrap();
        let response = Request::get(uri).execute().await.unwrap();
        assert_eq!(response.status().as_u16(), 200);
        println!("response.version: {:?}", response.version());
        println!("response.headers: {:?}", response.headers());

        let body = response.into_body();
        let resp = String::from_utf8(body).unwrap();
        println!("response.body: {}", resp);
    }
}

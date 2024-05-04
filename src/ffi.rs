#![allow(dead_code)]

pub mod binding {
    #![allow(warnings)]
    rust2go::r2g_include_binding!();
}

use crate::error::go_error::GoError;

macro_rules! impl_go_result {
    ($name:ident) => {
        impl $name {
            pub fn into_result(self) -> Result<(), crate::error::ErrorType> {
                if self.code == 0 {
                    Ok(())
                } else {
                    Err(GoError::from((self.code, self.message)).into())
                }
            }
        }
    };
    ($name:ident, $data:path) => {
        impl $name {
            pub fn into_result(self) -> Result<$data, crate::error::ErrorType> {
                if self.code == 0 {
                    Ok(self.data.into())
                } else {
                    Err(GoError::from((self.code, self.message)).into())
                }
            }
        }
    };
}

// === ReqwestxGoInit ===

#[rust2go::r2g]
pub trait ReqwestxGoInit {
    /// Prepare the client, you should call this function before any other functions
    ///
    /// # Pre-defined template
    /// - Chrome 0, will use utls.HelloChrome_106_Shuffle
    /// - Firefox 1, will use utls.HelloFirefox_105
    /// - Safari 2, will use utls.HelloSafari_16_0
    /// - Android11 Okhttp 10, will use utls.HelloAndroid_11_OkHttp, TODO: not implemented
    /// - Custom 255, will use utls.HelloCustom, TODO: not implemented
    ///
    /// If you want to use your own template, you can use `set_client_hello_id` to set the ClientHelloID,
    /// and provide any necessary setting
    #[send]
    #[drop_safe]
    fn init_client(config: ClientConfigFfi) -> GoResultFfi;

    /// Set proxy
    #[send]
    #[drop_safe]
    fn set_proxy_ffi(proxy: String) -> GoResultFfi;

    /// Manually set client impersonation config for adv usage
    #[send]
    #[drop_safe]
    fn update_impersonation_config(config: ImpersonationConfigFfi) -> GoResultFfi;

    /// Manually GC
    fn force_gc(data: bool) -> GoResultFfi;
}

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct GoResultFfi {
    code: i32,
    message: String,
}

impl_go_result!(GoResultFfi);

// === usage: init_client ===

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct ClientConfigFfi {
    /// Proxy address, accept http, https and socks5
    ///
    /// # Special value
    /// - `""` or `"direct"`: direct connection. Default value without init.
    /// - `"auto"` or `"env"`: auto detect proxy from env.
    pub proxy: String,
    /// basic impersonation config, only accept pre-defined template
    /// - Chrome 0, will use utls.HelloChrome_106_Shuffle
    /// - Firefox 1, will use utls.HelloFirefox_105
    /// - Safari 2, will use utls.HelloSafari_16_0
    pub impersonation_template: u8,
}

// === usage: update_impersonation_config ===

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct ImpersonationConfigFfi {
    /// TLS Fingerprint Impersonation
    pub utls_config: UTlsConfigFfi,
    /// HTTP2 Fingerprint Impersonation
    pub http2_settings_frame: Vec<Http2SettingFfi>,
    /// HTTP2 Connection flow
    pub http2_connection_flow: u32,
    /// Common Pseudo Header Order
    pub common_pseudo_header_order: Vec<String>,
    /// Common Header Order
    pub common_header_order: Vec<String>,
    /// Brower's common headers
    pub common_headers: Vec<HttpHeaderFfi>,
    /// HTTP2 Header Priority, for HTTP2 fingerprint fmpersonation
    pub http2_header_priority: Http2PriorityParamFfi,
}

// === UTLS Config ===

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct UTlsConfigFfi {
    pub id: ClientHelloIdFfi,
    pub spec: ClientHelloSpecFfi,
}

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct ClientHelloIdFfi {
    pub client: String,
    pub version: String,
}

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct ClientHelloSpecFfi {
    pub cipher_suites: Vec<u16>,
    pub compression_methods: Vec<u8>,
    pub extensions: Vec<TlsExtensionFfi>,
    pub tls_version_min: u16,
    pub tls_version_max: u16,
}

#[derive(Debug, Default, rust2go::R2G)]
#[repr(C)]
pub struct TlsExtensionFfi {
    pub ext_type: u16,
    pub vec_u32: Vec<u32>,
    pub vec_u16: Vec<u16>,
    pub vec_u8: Vec<u8>,
    pub vec_usize: Vec<usize>,
    pub vec_string: Vec<String>,
    pub data_isize: isize,
    pub data_bool: bool,
}

// === HTTP2 Fingerprint Config ===

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct Http2SettingFfi {
    pub setting_id: u16,
    pub setting_val: u32,
}

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct Http2PriorityParamFfi {
    pub stream_dep: u32,
    pub exclusive: bool,
    pub weight: u8,
}

// === ReqwestxGo ===

#[rust2go::r2g]
pub trait ReqwestxGo {
    #[send]
    #[drop_safe]
    fn send(req: HttpRequestFfi) -> impl std::future::Future<Output = GoResultHttpResponseFfi>;
}

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct HttpRequestFfi {
    pub url: String,
    pub method: u8,
    pub body: Vec<u8>,
    pub headers: Vec<HttpHeaderFfi>,
}

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct HttpHeaderFfi {
    pub k: String,
    pub v: Vec<String>,
}

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct HttpResponseFfi {
    /// StatusCode
    pub code: isize,
    /// Proto major
    pub proto_major: isize,
    /// Proto minor
    pub proto_minor: isize,
    /// Headers
    pub headers: Vec<HttpHeaderFfi>,
    /// Response data
    pub data: Vec<u8>,
}

#[derive(Debug, rust2go::R2G)]
#[repr(C)]
pub struct GoResultHttpResponseFfi {
    code: i32,
    message: String,
    data: HttpResponseFfi,
}

impl_go_result!(GoResultHttpResponseFfi, http::Response<Vec<u8>>);

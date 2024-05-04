pub mod impersonate;
pub use crate::ffi::ClientConfigFfi as ClientConfig;

use crate::{
    client::impersonate::ImpersonationConfig,
    ffi::{ReqwestxGoInit, ReqwestxGoInitImpl},
};

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

/// Update impersonation config
#[inline]
pub fn update_impersonation_config(config: ImpersonationConfig) {
    ReqwestxGoInitImpl::update_impersonation_config(config.into());
}

#[cfg(test)]
mod tests {
    use http::{HeaderMap, HeaderValue};

    use crate::{client::impersonate::*, client::*, request::Request};

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

    #[tokio::test]
    async fn test_custom_client() {
        let config = ClientConfig {
            proxy: "direct".to_string(),
            impersonation_template: 0xff,
        };
        init_client(config);

        let utls_spec = ClientHelloSpec::new()
            .set_cipher_suites(vec![
                GREASE_PLACEHOLDER, // TLS Grease
                TLS_AES_128_GCM_SHA256,
                TLS_AES_256_GCM_SHA384,
                TLS_CHACHA20_POLY1305_SHA256,
                TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
                TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
                TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
                TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
                TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305,
                TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305,
                TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA,
                TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA,
                TLS_RSA_WITH_AES_128_GCM_SHA256,
                TLS_RSA_WITH_AES_256_GCM_SHA384,
                TLS_RSA_WITH_AES_128_CBC_SHA,
                TLS_RSA_WITH_AES_256_CBC_SHA,
            ])
            .set_extensions(
                true,
                vec![
                    TlsExtension::TLSGrease,
                    TlsExtension::ServerName,
                    TlsExtension::ExtendedMasterSecret,
                    TlsExtension::RenegotiationInfo(1, vec![]),
                    TlsExtension::SupportedCurves(vec![
                        CurveID::Custom(GREASE_PLACEHOLDER), // TLS Grease
                        CurveID::X25519,
                        CurveID::CurveP256,
                        CurveID::CurveP384,
                    ]),
                    TlsExtension::SupportedPoints(vec![0x00]),
                    TlsExtension::SessionTicket,
                    TlsExtension::ALPN(vec!["h2", "http/1.1"]),
                    TlsExtension::StatusRequest,
                    TlsExtension::SignatureAlgorithms(vec![
                        SignatureScheme::ECDSAWithP256AndSHA256,
                        SignatureScheme::PSSWithSHA256,
                        SignatureScheme::PKCS1WithSHA256,
                        SignatureScheme::ECDSAWithP384AndSHA384,
                        SignatureScheme::PSSWithSHA384,
                        SignatureScheme::PKCS1WithSHA384,
                        SignatureScheme::PSSWithSHA512,
                        SignatureScheme::PKCS1WithSHA512,
                    ]),
                    TlsExtension::SCT,
                    TlsExtension::KeyShare(vec![
                        (CurveID::Custom(GREASE_PLACEHOLDER), vec![0x00]),
                        (CurveID::X25519, vec![]),
                    ]),
                    TlsExtension::PSKModes(vec![1]),
                    TlsExtension::SupportedVersions(vec![
                        GREASE_PLACEHOLDER,
                        TlsVersion::VersionTLS13 as u16,
                        TlsVersion::VersionTLS12 as u16,
                    ]),
                    TlsExtension::UtlsExtensionCompressCertificate(vec![0x0002]),
                    TlsExtension::UtlsExtensionApplicationSettings(vec!["h2".to_string()]),
                    TlsExtension::TLSGrease,
                    TlsExtension::UtlsExtensionPadding,
                ],
            );

        let utls_config = UTlsConfig {
            id: ClientHelloId::Custom,
            spec: Some(utls_spec),
        };

        let http2_settings_frame = vec![
            Http2Setting {
                setting_id: 0x1, // SettingHeaderTableSize
                setting_val: 65536,
            },
            Http2Setting {
                setting_id: 0x2, // SettingEnablePush
                setting_val: 0,
            },
            Http2Setting {
                setting_id: 0x3, // SettingMaxConcurrentStreams
                setting_val: 1000,
            },
            Http2Setting {
                setting_id: 0x4, // SettingInitialWindowSize
                setting_val: 6291456,
            },
            Http2Setting {
                setting_id: 0x6, // SettingMaxHeaderListSize
                setting_val: 262144,
            },
        ];

        let mut common_headers = HeaderMap::with_capacity(16);
        common_headers.insert("pragma", HeaderValue::from_static("no-cache"));
        common_headers.insert("cache-control", HeaderValue::from_static("no-cache"));
        common_headers.insert(
            "sec-ch-ua",
            HeaderValue::from_static(
                r#"Not_A Brand";v="124", "Google Chrome";v="124", "Chromium";v="124"#,
            ),
        );
        common_headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
        common_headers.insert("sec-ch-ua-platform", HeaderValue::from_static(r#"Linux"#));
        common_headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));
        common_headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"));
        common_headers.insert("accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
        common_headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
        common_headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
        common_headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
        common_headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
        common_headers.insert(
            "accept-language",
            HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,zh-TW;q=0.7,it;q=0.6"),
        );

        let impersonate_config = ImpersonationConfig {
            utls_config,
            http2_settings_frame,
            http2_connection_flow: 15663105,
            common_pseudo_header_order: vec![
                ":method".to_string(),
                ":authority".to_string(),
                ":scheme".to_string(),
                ":path".to_string(),
            ],
            common_header_order: vec![
                "host".to_string(),
                "pragma".to_string(),
                "cache-control".to_string(),
                "sec-ch-ua".to_string(),
                "sec-ch-ua-mobile".to_string(),
                "sec-ch-ua-platform".to_string(),
                "upgrade-insecure-requests".to_string(),
                "user-agent".to_string(),
                "accept".to_string(),
                "sec-fetch-site".to_string(),
                "sec-fetch-mode".to_string(),
                "sec-fetch-user".to_string(),
                "sec-fetch-dest".to_string(),
                "referer".to_string(),
                "accept-encoding".to_string(),
                "accept-language".to_string(),
                "cookie".to_string(),
            ],
            common_headers,
            http2_header_priority: Http2PriorityParam {
                exclusive: true,
                stream_dep: 0,
                weight: 255,
            },
        };

        update_impersonation_config(impersonate_config);

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

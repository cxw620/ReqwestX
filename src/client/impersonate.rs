use crate::ffi::{ClientHelloIdFfi, ClientHelloSpecFfi, TlsExtensionFfi, UTlsConfigFfi};

#[derive(Debug)]
pub struct UTlsConfig {
    pub id: ClientHelloId,
    pub spec: Option<ClientHelloSpec>,
}

impl From<UTlsConfig> for UTlsConfigFfi {
    fn from(value: UTlsConfig) -> Self {
        Self {
            id: ClientHelloIdFfi {
                client: value.id.client().to_string(),
                version: value.id.version().to_string(),
            },
            spec: value.spec.unwrap_or_default().into(),
        }
    }
}

#[derive(Debug)]
pub enum ClientHelloId {
    Firefox(&'static str),
    Chrome(&'static str),
    IOS(&'static str),
    Android11Okhttp(&'static str),
    Edge(&'static str),
    Safari(&'static str),
    Custom,
}

impl ClientHelloId {
    #[inline]
    pub fn client(&self) -> &'static str {
        match self {
            Self::Firefox(_) => "Firefox",
            Self::Chrome(_) => "Chrome",
            Self::IOS(_) => "iOS",
            Self::Android11Okhttp(_) => "Android",
            Self::Edge(_) => "Edge",
            Self::Safari(_) => "Safari",
            Self::Custom => "Custom",
        }
    }
    #[inline]
    pub fn version(&self) -> &'static str {
        match self {
            Self::Firefox(v) => v,
            Self::Chrome(v) => v,
            Self::IOS(v) => v,
            Self::Android11Okhttp(v) => v,
            Self::Edge(v) => v,
            Self::Safari(v) => v,
            Self::Custom => "0",
        }
    }
}

#[derive(Debug)]
pub struct ClientHelloSpec {
    cipher_suites: Vec<u16>,
    /// default compressionNone, vec![0]
    compression_methods: Vec<u8>,
    extensions: Vec<TlsExtension>,
    tls_version_min: TlsVersion,
    tls_version_max: TlsVersion,
}

impl Default for ClientHelloSpec {
    fn default() -> Self {
        Self {
            cipher_suites: Vec::new(),
            compression_methods: vec![0],
            extensions: Vec::with_capacity(32),
            tls_version_min: TlsVersion::VersionTLS12,
            tls_version_max: TlsVersion::VersionTLS13,
        }
    }
}

impl ClientHelloSpec {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_cipher_suite(mut self, ciper_suite: u16) -> Self {
        self.cipher_suites.push(ciper_suite);
        self
    }

    #[inline]
    pub fn add_cipher_suites(mut self, ciper_suites: impl IntoIterator<Item = u16>) -> Self {
        self.cipher_suites.extend(ciper_suites);
        self
    }

    #[inline]
    pub fn set_cipher_suites(mut self, ciper_suites: Vec<u16>) -> Self {
        self.cipher_suites = ciper_suites;
        self
    }

    #[inline]
    pub fn set_compression_method(mut self, compression_methods: Vec<u8>) -> Self {
        self.compression_methods = compression_methods;
        self
    }

    #[inline]
    pub fn add_extension(mut self, extension: TlsExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    #[inline]
    pub fn add_extensions(mut self, extensions: impl IntoIterator<Item = TlsExtension>) -> Self {
        self.extensions.extend(extensions);
        self
    }

    #[inline]
    pub fn set_extensions(mut self, extensions: Vec<TlsExtension>) -> Self {
        self.extensions = extensions;
        self
    }

    #[inline]
    pub fn set_tls_version_min(mut self, tls_version_min: TlsVersion) -> Self {
        self.tls_version_min = tls_version_min;
        self
    }

    #[inline]
    pub fn set_tls_version_max(mut self, tls_version_max: TlsVersion) -> Self {
        self.tls_version_max = tls_version_max;
        self
    }
}

impl From<ClientHelloSpec> for ClientHelloSpecFfi {
    fn from(value: ClientHelloSpec) -> Self {
        Self {
            cipher_suites: value.cipher_suites,
            compression_methods: value.compression_methods,
            extensions: value.extensions.into_iter().map(|ext| ext.into()).collect(),
            tls_version_min: value.tls_version_min as u16,
            tls_version_max: value.tls_version_max as u16,
        }
    }
}

// A list of cipher suite IDs that are, or have been, implemented by this
// package.
//
// See https://www.iana.org/assignments/tls-parameters/tls-parameters.xml

// TLS 1.0 - 1.2 cipher suites.
pub static TLS_RSA_WITH_RC4_128_SHA: u16 = 0x0005;
pub static TLS_RSA_WITH_3DES_EDE_CBC_SHA: u16 = 0x000a;
pub static TLS_RSA_WITH_AES_128_CBC_SHA: u16 = 0x002f;
pub static TLS_RSA_WITH_AES_256_CBC_SHA: u16 = 0x0035;
pub static TLS_RSA_WITH_AES_128_CBC_SHA256: u16 = 0x003c;
pub static TLS_RSA_WITH_AES_128_GCM_SHA256: u16 = 0x009c;
pub static TLS_RSA_WITH_AES_256_GCM_SHA384: u16 = 0x009d;
pub static TLS_ECDHE_ECDSA_WITH_RC4_128_SHA: u16 = 0xc007;
pub static TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA: u16 = 0xc009;
pub static TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA: u16 = 0xc00a;
pub static TLS_ECDHE_RSA_WITH_RC4_128_SHA: u16 = 0xc011;
pub static TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA: u16 = 0xc012;
pub static TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA: u16 = 0xc013;
pub static TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA: u16 = 0xc014;
pub static TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256: u16 = 0xc023;
pub static TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256: u16 = 0xc027;
pub static TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: u16 = 0xc02f;
pub static TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: u16 = 0xc02b;
pub static TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: u16 = 0xc030;
pub static TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: u16 = 0xc02c;
pub static TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: u16 = 0xcca8;
pub static TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: u16 = 0xcca9;

// TLS 1.3 cipher suites.
pub static TLS_AES_128_GCM_SHA256: u16 = 0x1301;
pub static TLS_AES_256_GCM_SHA384: u16 = 0x1302;
pub static TLS_CHACHA20_POLY1305_SHA256: u16 = 0x1303;

// TLS_FALLBACK_SCSV isn't a standard cipher suite but an indicator
// that the client is doing version fallback. See RFC 7507.
pub static TLS_FALLBACK_SCSV: u16 = 0x5600;

// Legacy names for the corresponding cipher suites with the correct _SHA256
// suffix, retained for backward compatibility.
pub static TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305: u16 = TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256;
pub static TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305: u16 =
    TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256;

#[derive(Debug)]
#[repr(u16)]
pub enum TlsExtension {
    ServerName(String) = 0,
    StatusRequest = 5,
    SupportedCurves(Vec<u16>) = 10, // supported_groups in TLS 1.3, see RFC 8446, Section 4.2.7
    SupportedPoints(Vec<u8>) = 11,
    SignatureAlgorithms(Vec<u16>) = 13,
    ALPN(Vec<String>) = 16,
    StatusRequestV2 = 17,
    SCT = 18,
    ExtendedMasterSecret = 23,
    SessionTicket = 35, // do not support customize its content
    SupportedVersions(Vec<u16>) = 43,
    PSKModes(Vec<u8>) = 45,
    SignatureAlgorithmsCert(Vec<SignatureScheme>) = 50,
    KeyShare(Vec<(u16, Vec<u8>)>) = 51,
    QUICTransportParameters = 57, // do not support customize its content
    RenegotiationInfo(isize, Vec<u8>) = 0xff01,

    // === Special ones ===
    UtlsExtensionPadding = 21, // do not support customize its content
    UtlsExtensionCompressCertificate(Vec<u16>) = 27, // https://datatracker.ietf.org/doc/html/rfc8879#section-7.1
    UtlsExtensionApplicationSettings(Vec<String>) = 17513, // not IANA assigned
    UtlsExtensionECH = 0xfe0d, // do not support customize its content, draft-ietf-tls-esni-17
    UtlsExtensionECHOuterExtensions = 0xfd00, // do not support customize its content, draft-ietf-tls-esni-17

    // extensions with 'fake' prefix break connection, if server echoes them back
    // FakeExtensionEncryptThenMAC = 22,
    FakeExtensionTokenBinding(u8, u8, Vec<u8>) = 24,
    FakeExtensionDelegatedCredentials(Vec<SignatureScheme>) = 34,
    FakeExtensionPreSharedKey = 41, // do not support customize its content
    FakeOldExtensionChannelID(bool) = 30031, // not IANA assigned
    FakeExtensionChannelID(bool) = 30032, // not IANA assigned

    // === Chrome specified ===
    TLSGrease(u16) = 0xFFFE,

    // === Custom One ===
    Custom(u16) = 0xFFFF,
    //**  === do not support ===
    // EarlyData = 42,
    // Cookie = 44,
    // CertificateAuthorities = 47,
    // UtlsFakeExtensionCustom = 1234,        // not IANA assigned, for ALPS
    //**
}

impl From<TlsExtension> for TlsExtensionFfi {
    fn from(ext: TlsExtension) -> TlsExtensionFfi {
        match ext {
            TlsExtension::ServerName(server_name) => Self {
                ext_type: 0,
                vec_string: vec![server_name],
                ..Default::default()
            },
            TlsExtension::StatusRequest => Self {
                ext_type: 5,
                ..Default::default()
            },
            TlsExtension::SupportedCurves(curves) => Self {
                ext_type: 10,
                vec_u16: curves,
                ..Default::default()
            },
            TlsExtension::SupportedPoints(points) => Self {
                ext_type: 11,
                vec_u8: points,
                ..Default::default()
            },
            TlsExtension::SignatureAlgorithms(schemes) => Self {
                ext_type: 13,
                vec_u16: schemes,
                ..Default::default()
            },
            TlsExtension::ALPN(protocols) => Self {
                ext_type: 16,
                vec_string: protocols,
                ..Default::default()
            },
            TlsExtension::StatusRequestV2 => Self {
                ext_type: 17,
                ..Default::default()
            },
            TlsExtension::SCT => Self {
                ext_type: 18,
                ..Default::default()
            },
            TlsExtension::ExtendedMasterSecret => Self {
                ext_type: 23,
                ..Default::default()
            },
            TlsExtension::SessionTicket => Self {
                ext_type: 35,
                ..Default::default()
            },
            TlsExtension::SupportedVersions(versions) => Self {
                ext_type: 43,
                vec_u16: versions,
                ..Default::default()
            },
            TlsExtension::PSKModes(modes) => Self {
                ext_type: 45,
                vec_u8: modes,
                ..Default::default()
            },
            TlsExtension::SignatureAlgorithmsCert(schemes) => Self {
                ext_type: 50,
                vec_u16: schemes.into_iter().map(|s| s as u16).collect(),
                ..Default::default()
            },
            TlsExtension::KeyShare(shares) => {
                let mut vec_u16 = Vec::with_capacity(shares.len());
                let mut vec_u8 = Vec::with_capacity(shares.len());
                let mut vec_usize = Vec::with_capacity(shares.len() * 3);

                shares.into_iter().for_each(|(group, keys)| {
                    vec_u16.push(group);
                    vec_usize.push(keys.len());
                    vec_u8.extend(keys);
                });

                Self {
                    ext_type: 51,
                    vec_u16,
                    vec_u8,
                    vec_usize,
                    ..Default::default()
                }
            }
            TlsExtension::QUICTransportParameters => Self {
                ext_type: 57,
                ..Default::default()
            },
            TlsExtension::RenegotiationInfo(len, data) => Self {
                ext_type: 0xff01,
                data_isize: len,
                vec_u8: data,
                ..Default::default()
            },
            TlsExtension::UtlsExtensionPadding => Self {
                ext_type: 21,
                ..Default::default()
            },
            TlsExtension::UtlsExtensionCompressCertificate(ids) => Self {
                ext_type: 27,
                vec_u16: ids,
                ..Default::default()
            },
            TlsExtension::UtlsExtensionApplicationSettings(settings) => Self {
                ext_type: 17513,
                vec_string: settings,
                ..Default::default()
            },
            TlsExtension::UtlsExtensionECH => Self {
                ext_type: 0xfe0d,
                ..Default::default()
            },
            TlsExtension::UtlsExtensionECHOuterExtensions => Self {
                ext_type: 0xfd00,
                ..Default::default()
            },
            TlsExtension::FakeExtensionTokenBinding(
                major_version,
                minor_version,
                key_parameters,
            ) => {
                let mut vec_u8 = Vec::with_capacity(key_parameters.len() + 2);
                vec_u8.push(major_version);
                vec_u8.push(minor_version);
                vec_u8.extend(key_parameters);
                Self {
                    ext_type: 24,
                    vec_u8,
                    ..Default::default()
                }
            }
            TlsExtension::FakeExtensionDelegatedCredentials(schemes) => Self {
                ext_type: 34,
                vec_u16: schemes.into_iter().map(|s| s as u16).collect(),
                ..Default::default()
            },
            TlsExtension::FakeExtensionPreSharedKey => Self {
                ext_type: 41,
                ..Default::default()
            },
            TlsExtension::FakeOldExtensionChannelID(enabled) => Self {
                ext_type: 30031,
                data_bool: enabled,
                ..Default::default()
            },
            TlsExtension::FakeExtensionChannelID(enabled) => Self {
                ext_type: 30032,
                data_bool: enabled,
                ..Default::default()
            },
            TlsExtension::TLSGrease(ext_type) => Self {
                ext_type,
                ..Default::default()
            },
            TlsExtension::Custom(ext_type) => Self {
                ext_type,
                ..Default::default()
            },
        }
    }
}

#[derive(Debug)]
#[repr(u16)]
pub enum SignatureScheme {
    // RSASSA-PKCS1-v1_5 algorithms.
    PKCS1WithSHA256 = 0x0401,
    PKCS1WithSHA384 = 0x0501,
    PKCS1WithSHA512 = 0x0601,

    // RSASSA-PSS algorithms with public key OID rsaEncryption.
    PSSWithSHA256 = 0x0804,
    PSSWithSHA384 = 0x0805,
    PSSWithSHA512 = 0x0806,

    // ECDSA algorithms. Only constrained to a specific curve in TLS 1.3.
    ECDSAWithP256AndSHA256 = 0x0403,
    ECDSAWithP384AndSHA384 = 0x0503,
    ECDSAWithP521AndSHA512 = 0x0603,

    // EdDSA algorithms.
    Ed25519 = 0x0807,

    // Legacy signature and hash algorithms for TLS 1.2.
    PKCS1WithSHA1 = 0x0201,
    ECDSAWithSHA1 = 0x0203,
}

#[derive(Debug)]
#[repr(u16)]
pub enum TlsVersion {
    VersionTLS10 = 0x0301,
    VersionTLS11 = 0x0302,
    VersionTLS12 = 0x0303,
    VersionTLS13 = 0x0304,

    // Deprecated: SSLv3 is cryptographically broken, and is no longer
    // supported by this package. See golang.org/issue/32716.
    VersionSSL30 = 0x0300,
}

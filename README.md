# ReqwestX

An easy and powerful Rust HTTP Client, based on [req](https://req.cool/).

*Supported HTTP Fingerprint Impersonation to Bypass Anti-Crawler Detection Effortlessly*

## Features

- [x] `ReqwestX` handles almost everything of FFI work with the help of [rust2go](https://github.com/ihciah/rust2go), making it really easy to use.

## Roadmap

- [x] Basic HTTP client functions
  - [x] proxy support(v0.1.0)
  - [ ] crate `http` compatibility(v0.1.0, partial)
- [x] Custom impersonation config including TLS, HTTP2 and browers fingerprint.(v0.1.1)
- [ ] ~~Pure Rust implementation based on crate `reqwest`~~
  
  Blocked, see [utls - issue#103](https://github.com/refraction-networking/utls/issues/103).

## License

MIT

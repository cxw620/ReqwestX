# Reqwest X

An ergonomic, batteries-included HTTP Client for Rust, which resembles the `reqwest` crate.

Currently this crate is still under development and may have breaking changes at any time,
not recommended for production usage.

## Features

- Basic HTTP Client features

  - [ ] Response Ext

  - [ ] Proxies Support (`socks5`, `http`, `https`, `custom`)

  - [ ] HTTPS via [`rustls`](https://crates.io/crates/rustls) or [system-native TLS](https://crates.io/crates/native-tls). Default to `rustls` fully written in Rust.

  - [ ] Cookie Store

- Advanced features

    Some advanced features that may not supported by `reqwest`.

  - [ ] OpenTelemetry support

  - [ ] TLS impersonation

    Reference: [`req`](https://req.cool/zh/docs/tutorial/http-fingerprint/)

## Examples

WIP

## License

GPL-3.0-only

## Thanks

This crate is highly inspired by `reqwest`. Thanks to the authors and contributors of `reqwest`.

TLS impersonation is powered by [`req`](https://req.cool/zh/docs/tutorial/http-fingerprint/).

## TODOs

- [ ] Android Okhttp TLS characteristics(ja3, ja4)
- [ ] Android cornet TLS characteristics(ja3, ja4)

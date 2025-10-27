# nvi_rs

Rust client for NVI / KPS identity verification — async, small and production-oriented.

`nvi_rs` is a lightweight Rust library that helps interacting with the Turkish NVI/KPS SOAP identity verification services. It includes helpers to build SOAP requests, obtain WS-Trust (STS) tokens, sign requests when required, and parse responses into ergonomic Rust types.

## Highlights

- Async API (tokio + reqwest)
- WS-Trust (STS) token acquisition
- Small, robust XML helpers (quick-xml)
- Enterprise-style constants exposed in `crate::constants`

## Cross-platform single-command run

The example uses an optional `.env` file (loaded with `dotenvy`) so you can run it the same way on Windows, macOS and Linux.

Create a `.env` file in the project root with the following (optional) values:

```
KPS_USERNAME=your-username
KPS_PASSWORD=your-password
```

Then run the example from the project root with a single command:

```
cargo run --example basic
```

This approach avoids OS-specific environment commands and works cross-platform.

## Quick start

1. Install Rust (rustup) — https://rustup.rs
2. (Optional) Create a `.env` with credentials as shown above.
3. Run the example:

```
cargo run --example basic
```

## Run tests

Run the full test suite (includes an integration test that uses wiremock):

```
cargo test --workspace --verbose
```
<div align="center">
    <h1>wom_rs</h1>
</div>

A asynchronous rust wrapper for interacting with [Wise Old Man API](https://docs.wiseoldman.net/).

The goal of wom_rs is to provide a 1:1 in rust to their API. 

## Getting Started

add `wom_rs` to your `Cargo.toml` file:

```toml
[dependencies]
wom_rs = "0.1.0"
```

To get a client for interacting with the API, use the `WomClient` struct:

```rust
use wom_rs::WomClient;

//Default client
let wom_client = WomClient::new();

//Client with an api key
let api_key = std::env::var("WOM_API_KEY").unwrap();
let wom_client = WomClient::new_with_key(api_key);
```
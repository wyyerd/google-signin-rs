Google Sign-In for Rust
=======================

[![google-signin on crates.io](https://img.shields.io/crates/v/google-signin.svg)](https://crates.io/crates/google-signin)
[![google-signin on docs.rs](https://docs.rs/google-signin/badge.svg)](https://docs.rs/google-signin)

Rust API bindings for Google Sign-in.  
See [authenticating with a backend server](https://developers.google.com/identity/sign-in/web/backend-auth).

## Usage
Put this in your `Cargo.toml`:

```toml
[dependencies]
google-signin = "0.3.0"
```

And this in your crate root:

```rust
extern crate google_signin;
```

And then you can verify a google JSON web token

```rust
use google_signin;
let mut client = google_signin::Client::new();
client.audiences.push(YOUR_CLIENT_ID); // required
client.hosted_domains.push(YOUR_HOSTED_DOMAIN); // optional

// Let the crate handle everything for you
let id_info = client.verify(&data.token).expect("Expected token to be valid");
println!("Success! Signed-in as {}", id_info.sub);

// Inspect the ID before verifying it
let id_info = client.get_slow_unverified(&data.token).expect("Expected token to exist");
let ok = id_info.verify(&client).is_ok();
println!("Ok: {}, Info: {:?}", ok, id_info);
```

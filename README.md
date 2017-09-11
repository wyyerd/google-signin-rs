google-oauth2-rs
=========

Rust bindings for the Google OAuth2 Server API

## Usage
Put this in your `Cargo.toml`:

```toml
[dependencies]
google-oauth2 = { git = "ssh://git@git.rapiditynetworks.com/rapidity/google-oauth2-rs.git" }
```

And this in your crate root:

```rust
extern crate google_oauth2;
```

And then you can verify a google oauth2 token

```rust
use google_oauth2 as gapi;
let auth2 = gapi::Client::new();
let token = gapi::TokenInfo::get(&auth2, &data.token).expect("Expected token to be valid");
match token.iss.as_str() {
    "accounts.google.com" | "https://accounts.google.com" => {}
    _ => { panic!("Expected token to be issued by Google"); }
}
if token.aud != YOUR_CLIENT_ID {
    panic!("Expected token to be for this OAuth application");
}

// To restrict login to a specific company / hosted-domain
let token_domain = token.hd.ok_or_else(|| {
  panic!("Expected token to have a Hosted Domain")
});
if token_domain != YOUR_HOSTED_DOMAIN {
  panic!("Expected token to be in the same Hosted Domain");
}

println!("Success!");
```

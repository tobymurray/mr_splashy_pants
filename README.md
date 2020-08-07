![Rust](https://github.com/tobymurray/mr_splashy_pants/workflows/Rust/badge.svg)
[![Documentation](https://img.shields.io/badge/documentation-available-green.svg)](https://docs.rs/crate/mr_splashy_pants/)
[![Crate](https://img.shields.io/crates/v/mr_splashy_pants.svg)](https://crates.io/crates/mr_splashy_pants)

# This is a WIP, don't use it

Set up a script with access to a Reddit account, collect the access token, the client ID, and the client secret. Once you have that, get a refresh token. Once you have that you can do:

```
// Refresh token is mutable so it can be refreshed automatically
let mut refresh_token = "<refresh-token>";

let pants = Pants::new(
    USER_AGENT,
    "<access-token>",
    "<client-id>",
    "<client-secret>",
);
```
Then you can invoke things, e.g:

```
pants.me(&mut refresh_token)
```

If your refresh token expires, it should automatically refresh.

![Rust](https://github.com/tobymurray/mr_splashy_pants/workflows/Rust/badge.svg)
[![Documentation](https://img.shields.io/badge/documentation-available-green.svg)](https://docs.rs/crate/mr_splashy_pants/)
[![Crate](https://img.shields.io/crates/v/mr_splashy_pants.svg)](https://crates.io/crates/mr_splashy_pants)

# This is a WIP, don't use it
Follow https://github.com/reddit-archive/reddit/wiki/OAuth2 for set up instructions.

Set up a script with access to a Reddit account, collect the access token, the client ID, and the client secret. Once you have that, get a refresh token. Once you have that you can do:

```
// pants is mutable so the refresh token can be updated
let mut pants = Pants::new(
    USER_AGENT,
    "<access-token>",
    "<refresh_token>",
    "<client-id>",
    "<client-secret>",
);
```
For example, if you're using dotenv and reading values from the environment:
```
// pants is mutable so the refresh token can be updated
let mut pants = Pants::new(
    USER_AGENT,
    &env::var("ACCESS_TOKEN").unwrap(),
    env::var("REFRESH_TOKEN").unwrap(),
    &env::var("CLIENT_ID").unwrap(),
    &env::var("CLIENT_SECRET").unwrap(),
);
```
Then you can invoke things, e.g:

```
pants.me()
```

If your refresh token expires, it should automatically refresh.

So far there are precisely two APIs implemented:
- [/api/v1/me](https://www.reddit.com/dev/api#GET_api_v1_me) (with partially structured response)
- [/api/v1/me/trophies](https://www.reddit.com/dev/api#GET_api_v1_me_trophies) (with JSON response)

All other APIs are not implemented
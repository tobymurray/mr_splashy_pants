use reqwest::Client;

// API is: '/api/v1/me'
pub fn api_v1_me(client: reqwest::Client) {
  // client.get("")
  println!("/api/v1/me");
}

// API is: '/api/v1/me/blocked'
pub fn api_v1_me_blocked() {
  println!("/api/v1/me/blocked");
}

// API is: '/api/v1/me/friends'
pub fn api_v1_me_friends() {
  println!("/api/v1/me/friends");
}

// API is: '/api/v1/me/karma'
pub fn api_v1_me_karma() {
  println!("/api/v1/me/karma");
}

// API is: '/api/v1/me/prefs'
pub fn api_v1_me_prefs() {
  println!("/api/v1/me/prefs");
}

// API is: '/api/v1/me/trophies'
pub fn api_v1_me_trophies() {
  println!("/api/v1/me/trophies");
}

// API is: '/prefs/blocked'
pub fn prefs_blocked() {
  println!("/prefs/blocked");
}

// API is: '/prefs/friends'
pub fn prefs_friends() {
  println!("/prefs/friends");
}

// API is: '/prefs/messaging'
pub fn prefs_messaging() {
  println!("/prefs/messaging");
}

// API is: '/prefs/trusted'
pub fn prefs_trusted() {
  println!("/prefs/trusted");
}

// API is: '/prefs/'
pub fn prefs() {
  println!("/prefs/");
}


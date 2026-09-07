use std::fmt::Display;

use uz_types::{AccessToken, ClientSecret, RefreshToken};

fn assert_display<T: Display>() {}

fn main() {
    assert_display::<AccessToken>();
    assert_display::<RefreshToken>();
    assert_display::<ClientSecret>();
}

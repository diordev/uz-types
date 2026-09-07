use serde::Serialize;
use uz_types::{AccessToken, ClientSecret, RefreshToken};

fn assert_serialize<T: Serialize>() {}

fn main() {
    assert_serialize::<AccessToken>();
    assert_serialize::<RefreshToken>();
    assert_serialize::<ClientSecret>();
}

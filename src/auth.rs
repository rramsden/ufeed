extern crate crypto;
extern crate jwt;

use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{
    Header,
    Registered,
    Token,
};

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
  pub token: String
}

pub fn new_token(user_id: &str, password: &str) -> String {
    let header: Header = Default::default();
    let claims = Registered {
        sub: Some(user_id.into()),
        ..Default::default()
    };
    let token = Token::new(header, claims);

    token.signed(b"secret_key", Sha256::new()).ok()
}

pub fn login(token: &str) -> Option<String> {
    let token = Token::<Header, Registered>::parse(token).unwrap();

    if token.verify(b"secret_key", Sha256::new()) {
        token.claims.sub
    } else {
        None
    }
}

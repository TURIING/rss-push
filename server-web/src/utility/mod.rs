use crate::types::auth::Claims;

use crate::constant::{JWTSECRET, JWTALG};
use crate::error::RssError;

use jsonwebtoken::{ EncodingKey, DecodingKey, Validation };

pub struct Jwt;
impl Jwt {
    // return jwt token
    pub fn form(username: String, passwd: String) -> Result<String, RssError> {
        let claims = Claims { username, passwd, ..Default::default() };
        //let header = jsonwebtoken::Header { alg: JWTALG, ..Default::default() };
        let header = jsonwebtoken::Header::new(JWTALG);

        match jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWTSECRET)) {
            Ok(token) => {
                Ok(token)
            },
            Err(e) => Err(RssError::JwtInvalidError(e))
        }
    }
    // return username
    pub fn validate(token: String) -> Result<String, RssError> {
        match jsonwebtoken::decode::<Claims>(
            &token, 
            &DecodingKey::from_secret(JWTSECRET), 
            &Validation::new(JWTALG)
        ) {
            Ok(token_data) => Ok(token_data.claims.username),
            Err(e) => Err(RssError::JwtInvalidError(e)),
        }
    }
}
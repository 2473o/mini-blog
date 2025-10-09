use jwt_simple::{claims::Claims, prelude::*};

use crate::dto::User;
use crate::state::TokenVeirfy;

use axum::{
    extract::{FromRequestParts, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

const JWT_DURATION: u64 = 64 * 64 * 24 * 7;
const JWT_ISS: &str = "mini-blog";
const JWT_AUD: &str = "blog-users";

#[derive(Clone)]
pub struct EncodingKey(RS256KeyPair);

#[derive(Debug, Clone)]
pub struct DecodingKey(RS256PublicKey);

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(RS256KeyPair::from_pem(pem)?))
    }

    pub fn sign(&self, user: User) -> Result<String, jwt_simple::Error> {
        let claims: JWTClaims<User> =
            Claims::with_custom_claims(user, Duration::from_secs(JWT_DURATION));

        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        self.0.sign(claims)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(RS256PublicKey::from_pem(pem)?))
    }

    pub fn verify(&self, token: &str) -> Result<User, jwt_simple::Error> {
        let opts = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };

        let claims = self.0.verify_token::<User>(token, Some(opts))?;
        Ok(claims.custom)
    }
}

pub async fn verify_token<T>(State(state): State<T>, req: Request, next: Next) -> Response
where
    T: TokenVeirfy + Clone + Send + Sync + 'static,
{
    let (mut parts, body) = req.into_parts();
    let token =
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await {
            Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
            Err(e) => {
                let msg = format!("parse authorization error: {}", e);
                return (StatusCode::UNAUTHORIZED, msg).into_response();
            }
        };

    let req = match state.vetify(&token) {
        Ok(user) => {
            let mut req = Request::from_parts(parts, body);
            req.extensions_mut().insert(user);
            req
        }
        Err(e) => {
            let msg = format!("verify token failed: {:?}", e);
            return (StatusCode::FORBIDDEN, msg).into_response();
        }
    };

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_generate_keys() -> Result<()> {
        let ek = EncodingKey::load(include_str!("../keys/private.pem"))?;
        let dk = DecodingKey::load(include_str!("../keys/public.pem"))?;

        let user = User {
            id: 1,
            username: "john".to_string(),
            display_name: "John".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let token = ek.sign(user.clone())?;
        println!("sign token: {:?}", token);

        let verify_user = dk.verify(&token)?;
        println!("verify_user: {:?}", verify_user);

        assert_eq!(user, verify_user);
        Ok(())
    }
}

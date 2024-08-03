use std::sync::{Arc, Mutex};
use std::time::Duration;
use base64url::{decode, encode};
use hmac::{Hmac, Mac};
/// Create and store JWT
/// ChatGLM support two kinds of authentication: API Key and JWT.
/// details: https://open.bigmodel.cn/dev/api#http_auth
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use sha2::Sha256;
use crate::glm_client::ApiKey;
use crate::util;

lazy_static! {
    static ref JWT_HOLDER: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("")));
}

pub(crate) fn get_or_create(api_key: &ApiKey) -> String {
    get_or_create_with_expired(api_key, None)
}

pub(crate) fn get_or_create_with_expired(api_key: &ApiKey, expire_after: Option<Duration>) -> String {
    let mut token = JWT_HOLDER.lock().unwrap();
    let token_ref = &*token;
    // reuse token
    if !token_ref.is_empty() && !check_expired(token_ref) {
        return token_ref.clone();
    }
    // create new token
    let mut jwt_builder = JwtBuilder::new(api_key);
    if let Some(expire_after) = expire_after {
        jwt_builder.expire_after(expire_after);
    }
    let new_token = jwt_builder.build();
    *token = new_token.clone();
    new_token
}

fn check_expired(token: &str) -> bool {
    let jwt = token.trim();
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return true;
    }
    let payload_decoded = decode(parts[1]).unwrap();
    let payload_json = std::str::from_utf8(&payload_decoded).unwrap();
    let payload: Payload = serde_json::from_str(payload_json).unwrap();
    payload.exp < util::current()
}

pub fn verify(secret: String, jwt: &str) -> bool {
    let jwt = jwt.trim();
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    let encoded_header = parts[0];
    let encoded_payload = parts[1];
    let signature = parts[2];
    let to_verify = format!("{}.{}", encoded_header, encoded_payload);
    let calculated_signature_bytes = generate_signature(secret, &to_verify);
    let calculated_signature = encode(&calculated_signature_bytes);

    calculated_signature == signature
}

pub(crate) struct JwtBuilder {
    secret: String,
    header: String,
    payload: Payload,
}

impl JwtBuilder {
    fn new(api_key: &ApiKey) -> JwtBuilder {
        let header = "{\"alg\":\"HS256\",\"sign_type\":\"SIGN\"}".to_string();
        let payload = Payload::new(api_key.user_id().to_string());
        JwtBuilder {
            secret: api_key.secret_key().to_string(),
            header,
            payload,
        }
    }

    fn expire_after(&mut self, duration: Duration) -> &mut JwtBuilder {
        self.payload.exp = util::current() + duration.as_millis() as u64;
        self
    }

    fn build(&self) -> String {
        let encoded_header = encode(self.header.as_bytes());
        let encoded_payload = encode(self.payload.as_bytes());
        let to_sign = format!("{}.{}", encoded_header, encoded_payload);

        let signature_bytes = self.generate_signature(&to_sign);
        let calculated_signature = encode(&signature_bytes);
        format!("{}.{}", to_sign, calculated_signature)
    }

    fn generate_signature(&self, data: &str) -> Vec<u8> {
        generate_signature(self.secret.clone(), data)
    }
}

fn generate_signature(secret: String, data: &str) -> Vec<u8> {
    let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("HMAC key error");
    hmac.update(data.as_bytes());
    hmac.finalize().into_bytes().to_vec()
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Payload {
    api_key: String,
    exp: u64,
    timestamp: u64,
}

impl Payload {
    fn new(api_key: String) -> Payload {
        let current = util::current();
        Payload {
            api_key,
            exp:  current + 12 * 3600 * 1000,
            timestamp: current,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use crate::glm_client::ApiKey;
    use crate::jwt::get_or_create_with_expired;

    #[test]
    fn jwt_holder() {
        let api_key = ApiKey::from_string("xxx.yyy".to_string()).unwrap();
        let jwt = get_or_create_with_expired(&api_key, Some(Duration::from_secs(5)));
        println!("{jwt}");
        let jwt2 = get_or_create_with_expired(&api_key, Some(Duration::from_secs(5)));
        assert_eq!(jwt2, jwt);
        // token expired after 5 seconds, will create new token
        thread::sleep(Duration::from_secs(6));
        let jwt3 = get_or_create_with_expired(&api_key, Some(Duration::from_secs(5)));
        assert_ne!(jwt3, jwt);
    }
}
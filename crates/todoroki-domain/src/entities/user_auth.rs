use crate::value_object;

value_object!(UserAuthToken(String));

pub struct VerificationKey {
    key: jsonwebtoken::DecodingKey,
}

impl VerificationKey {
    pub fn new(key: jsonwebtoken::DecodingKey) -> Self {
        Self { key }
    }

    pub fn value(&self) -> &jsonwebtoken::DecodingKey {
        &self.key
    }
}

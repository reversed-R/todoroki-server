use crate::value_object;
use getset::Getters;

#[derive(Debug, Clone, Getters)]
pub struct VerificationKey {
    #[getset(get = "pub")]
    key: UserAuthKey,
}

value_object!(UserAuthKey(jsonwebtoken::DecodingKey));

impl VerificationKey {
    pub fn new(key: UserAuthKey) -> Self {
        Self { key }
    }
}

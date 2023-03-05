use anyhow::Result;
use jwt_simple::prelude::*;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::{constants::MOJANG_PUBKEY, error::LoginError};

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTHeader {
    x5u: String,
    alg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthChain {
    chain: Vec<String>,
}
#[derive(Debug)]
pub struct VerifiedData {
    pub key: String,
    pub user_data: ExtraUserdata,
}

pub fn verify<CustomClaim: Serialize + DeserializeOwned>(
    chain: String,
    verified: &mut bool,
) -> Result<CustomClaim> {
    let header: JWTHeader = serde_json::from_slice(
        &base64::decode(chain.split(".").collect::<Vec<&str>>()[0]).unwrap(),
    )
    .unwrap();
    if header.x5u == MOJANG_PUBKEY {
        *verified = true;
    }
    let key: ES384PublicKey =
        ES384PublicKey::from_der(&base64::decode(&header.x5u).unwrap()).unwrap();
    let token: JWTClaims<CustomClaim> = key.verify_token::<CustomClaim>(&chain, None)?;
    Ok(token.custom)
}

pub fn verify_auth(chains: &str) -> Result<VerifiedData> {
    let raw_chains: AuthChain = serde_json::from_str(chains)?;
    let chains: Vec<String> = raw_chains.chain.clone();
    if chains.len() != 3 {
        return Err(LoginError::InvalidChainLength(chains.len()).into());
    }
    let mut verified: bool = false;
    let mut result: Option<(ExtraUserdata, String)> = None;

    for chain in chains {
        let claim: AdditionalData = verify::<AdditionalData>(chain, &mut verified)?;
        if let Some(data) = claim.extraData {
            result = Some((data, claim.identityPublicKey));
        }
    }
    if verified == false {
        return Err(LoginError::NotAuthenticated.into());
    }
    match result {
        Some((user_data, key)) => Ok(VerifiedData { key, user_data }),
        None => Err(LoginError::WrongClaim.into()),
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct AdditionalData {
    extraData: Option<ExtraUserdata>,
    certificateAuthority: Option<bool>,
    randomNonce: Option<isize>,
    identityPublicKey: String,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ExtraUserdata {
    pub XUID: String,
    pub identity: String,
    pub displayName: String,
    pub titleId: String,
}

pub fn verify_skin(public_key: &str, client: &str) -> Result<Value> {
    let key: ES384PublicKey =
        ES384PublicKey::from_der(&base64::decode(public_key).unwrap()).unwrap();
    match key.verify_token::<Value>(client, None) {
        Ok(v) => Ok(v.custom),
        Err(_) => Err(LoginError::WrongSkinData.into()),
    }
}

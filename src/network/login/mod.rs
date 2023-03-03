use std::io::Result;

use serde::{Deserialize, Serialize};
use jwt_simple::prelude::*;
use super::packet::Login;

pub fn decode_login_jwt(login: Login) -> Result<()> {
    let authchain: AuthChain = serde_json::from_str(&login.identity)?;
    let chain_data = authchain.chain.clone();
    for chain in chain_data {
        decode_chain(chain);
    }
    Ok(())
}

const MOJANG_PUBKEY :&str="MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE8ELkixyLcwlZryUQcu1TvPOmI2B7vX83ndnWRUaXm74wFfa5f/lwQNTfrLVHa2PmenpGI6JhIMUJaWZrjmMj90NoKNFSNBuKdm8rYiXsfaz3K36x/1U26HpG0ZxK/V1V";

pub fn decode_chain(chain: String) {
    let header: JWTHeader = serde_json::from_slice(
        &base64::decode(chain.split(".").collect::<Vec<&str>>()[0]).unwrap(),
    )
    .unwrap();
    let key = ES384PublicKey::from_der(&base64::decode(&header.x5u).unwrap()).unwrap();
    match key.verify_token::<AdditionalData>(&chain,None) {
        Ok(claim) => {
            println!("{:?}",claim.custom.extraData);
        },
        Err(e) => {
            println!("{}",e);
        }
    }
}
#[derive(Debug)]
pub enum LoginError {
    InvalidChain,
}
#[allow(non_snake_case)]
#[derive(Deserialize,Serialize,Debug)]
pub struct AdditionalData {
    extraData:ExtraUserdata,
    identityPublicKey:String,
    randomNonce:isize
}

#[allow(non_snake_case)]
#[derive(Deserialize,Serialize,Debug)]
pub struct ExtraUserdata {
    XUID:String,
    identity:String,
    displayName:String,
    titleId:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTHeader {
    x5u:String,
    alg:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthChain {
    chain: Vec<String>,
}

mod constants;
mod error;
mod login_verify;
use anyhow::Result;

use super::packet::Login;
use crate::network::login::login_verify::{verify_auth, verify_skin};

pub fn decode_login_jwt(login: Login) -> Result<()> {
    let v = verify_auth(&login.identity)?;
    let skin_data = verify_skin(&v.key, &login.client)?;
    println!("{:?}", v);
    Ok(())
}

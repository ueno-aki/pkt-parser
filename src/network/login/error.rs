use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Invalid chains length. found:{0},expected:3")]
    InvalidChainLength(usize),
    #[error("Not Authenticated.")]
    NotAuthenticated,
    #[error("Wrong claim.")]
    WrongClaim,
    #[error("Wrong SkinData claim.")]
    WrongSkinData,
}

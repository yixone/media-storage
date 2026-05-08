pub mod asset_repo;
pub mod media_repo;

pub use asset_repo::*;
pub use media_repo::*;

/// Database repository Master trait
pub trait RepostiryExt: Send + Sync + AssetRepoExt + MediaRepoExt {}

/// Supertrait blanket implementation
impl<T: Send + Sync + AssetRepoExt + MediaRepoExt> RepostiryExt for T {}

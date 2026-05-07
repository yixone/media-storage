pub mod asset_repo;
pub mod collection_repo;
pub mod media_repo;

pub use asset_repo::*;
pub use collection_repo::*;
pub use media_repo::*;

/// Database repository Master trait
pub trait RepostiryExt: Send + Sync + AssetRepoExt + CollectionRepoExt + MediaRepoExt {}

/// Supertrait blanket implementation
impl<T: Send + Sync + AssetRepoExt + CollectionRepoExt + MediaRepoExt> RepostiryExt for T {}

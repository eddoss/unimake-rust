pub mod global;
pub mod kit;
pub mod umk;

pub mod prelude {
    pub use super::umk::Error;
    pub use super::umk::Result;
    pub use super::*;

    pub use super::workspace::Mode as WorkspaceMode;
    pub use super::workspace::Workspace;

    pub use super::workspace::Cache;
    pub use super::workspace::CacheDriver;
    pub use super::workspace::CacheEntry;
    pub use super::workspace::CacheKeys;
    pub use super::workspace::FsCacheDriver;
}

mod workspace;

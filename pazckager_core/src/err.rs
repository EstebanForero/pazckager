use thiserror::Error;

use crate::traits::{StoreError, ToolError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Installer(#[from] ToolError),
    #[error("Store error: {0}")]
    Store(#[from] StoreError),
    #[error("Package already exists")]
    PackageAlreadyExists,
    #[error("Package does not exists")]
    PackageDoesNotExists,
    #[error("Installation tool does not exist")]
    InstallationToolDoesNotExist,
    #[error("Category does not exists")]
    CategoryDoesNotExist,
}

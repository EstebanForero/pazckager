use thiserror::Error;

use crate::models::{Category, InstallationTools, PackageData, RawPackageData};

pub trait PazckagerStorage {
    fn store_category(&mut self, category: Category) -> StoreResult<()>;

    fn get_categories(&self) -> StoreResult<Vec<Category>>;

    fn category_exists(&self, category_name: &str) -> StoreResult<bool>;

    fn remove_category(&mut self) -> StoreResult<()>;

    fn store_package(&mut self, package: PackageData) -> StoreResult<()>;

    fn update_package(&mut self, package: PackageData) -> StoreResult<()>;

    fn get_packages(&self) -> StoreResult<Vec<PackageData>>;

    fn package_exists(&self, package_name: &str) -> StoreResult<bool>;

    fn get_package(&self, package_name: &str) -> StoreResult<PackageData>;

    fn get_packages_by_category(&self, category_name: &str) -> StoreResult<Vec<PackageData>>;

    fn remove_package(&mut self, package_name: &str) -> StoreResult<()>;
}

pub type StoreResult<T> = Result<T, StoreError>;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("{0}")]
    InternalStoreError(String),
}

pub trait InstallationTool {
    fn get_type(&self) -> InstallationTools;

    fn install_package(&mut self, package_name: &str) -> ToolResult<()>;

    fn delete_package(&mut self, package_name: &str) -> ToolResult<()>;

    fn update_package(&mut self, package_name: &str) -> ToolResult<()>;

    fn get_packages(&self) -> Vec<RawPackageData>;
}

pub type ToolResult<T> = Result<T, ToolError>;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("Error installing package: {0}")]
    InstallingPackage(String),
    #[error("Error deleting package: {0}")]
    DeletingPackage(String),
    #[error("Error updating package: {0}")]
    UpdatingPackage(String),
}

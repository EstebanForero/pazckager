use std::collections::HashMap;

use models::{Category, InstallationTools, PackageData};
use traits::{InstallationTool, PazckagerStorage};

pub mod err;
pub mod models;
pub mod traits;

use err::{Error, Result};

pub struct PazckagerCore<T: PazckagerStorage> {
    store: T,
    package_installers: HashMap<InstallationTools, Box<dyn InstallationTool>>,
}

impl<T: PazckagerStorage> PazckagerCore<T> {
    fn new(
        mut store: T,
        package_installers: HashMap<InstallationTools, Box<dyn InstallationTool>>,
    ) -> Result<Self> {
        if !store.category_exists("default")? {
            store.store_category(Category {
                category_name: "default".to_string(),
                additional_info: None,
            })?;
        }

        Ok(Self {
            store,
            package_installers,
        })
    }

    pub fn add_package(
        &mut self,
        package_name: String,
        installation_tool: Option<InstallationTools>,
        package_category_name: Option<String>,
    ) -> Result<()> {
        let package_installer = if let Some(installation_tool) = installation_tool {
            self.package_installers
                .get_mut(&installation_tool)
                .ok_or(Error::InstallationToolDoesNotExist)?
        } else {
            self.package_installers.values_mut().next().unwrap()
        };

        let mut category_name = package_category_name
            .clone()
            .unwrap_or("default".to_string());

        category_name = if category_name.is_empty() {
            "default".to_string()
        } else {
            category_name
        };

        if !self.store.category_exists(&category_name)? {
            return Err(Error::CategoryDoesNotExist);
        }

        if self.store.package_exists(&package_name)? {
            return Err(Error::PackageAlreadyExists);
        }

        if let Err(err) = package_installer.install_package(&package_name) {
            return Err(err.into());
        } else {
            self.store.store_package(PackageData {
                package_name,
                instalation_tool: package_installer.get_type(),
                category_name,
            })?;
        }

        Ok(())
    }

    pub fn delete_package(&mut self, package_name: String) -> Result<()> {
        if !self.store.package_exists(&package_name)? {
            return Err(Error::PackageDoesNotExists);
        }

        let package = self.store.get_package(&package_name)?;

        let package_installer = self
            .package_installers
            .get_mut(&package.instalation_tool)
            .ok_or(Error::InstallationToolDoesNotExist)?;

        package_installer.delete_package(&package.package_name)?;

        Ok(())
    }

    pub fn update_package(&mut self, package_name: String) -> Result<()> {
        if !self.store.package_exists(&package_name)? {
            return Err(Error::PackageDoesNotExists);
        }

        let package = self.store.get_package(&package_name)?;

        let package_installer = self
            .package_installers
            .get_mut(&package.instalation_tool)
            .ok_or(Error::InstallationToolDoesNotExist)?;

        package_installer.update_package(&package.package_name)?;

        Ok(())
    }

    pub fn get_packages(&self) -> Result<Vec<PackageData>> {
        Ok(self.store.get_packages()?)
    }

    pub fn get_categories(&self) -> Result<Vec<Category>> {
        Ok(self.store.get_categories()?)
    }

    pub fn get_package_by_category(&self, category_name: String) -> Result<Vec<PackageData>> {
        Ok(self.store.get_packages_by_category(&category_name)?)
    }
}

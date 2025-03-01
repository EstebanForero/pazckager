use pazckager_core::traits::{StoreError, StoreResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

use models::{Category, PackageData};
use pazckager_core::*;
use traits::{PazckagerStorage, StoreError as Error};

#[derive(Serialize, Deserialize, Default)]
struct JsonStore {
    categories: HashMap<String, Category>,
    packages: HashMap<String, PackageData>,
}

pub struct JsonPazckagerStorage {
    store: JsonStore,
    file_path: PathBuf,
}

impl JsonPazckagerStorage {
    pub fn new(file_path: impl Into<PathBuf>) -> StoreResult<Self> {
        let file_path = file_path.into();

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| Error::InternalStoreError(e.to_string()))?;
        }

        let store = if file_path.exists() {
            let file =
                File::open(&file_path).map_err(|e| Error::InternalStoreError(e.to_string()))?;
            serde_json::from_reader(file).map_err(|e| Error::InternalStoreError(e.to_string()))?
        } else {
            JsonStore::default()
        };
        Ok(Self { store, file_path })
    }

    fn save_to_file(&self) -> StoreResult<()> {
        if self.file_path.exists() {
            fs::remove_file(&self.file_path)
                .map_err(|e| StoreError::InternalStoreError(e.to_string()))?;
        }
        let file = File::create(&self.file_path)
            .map_err(|e| StoreError::InternalStoreError(e.to_string()))?;
        serde_json::to_writer_pretty(file, &self.store)
            .map_err(|e| StoreError::InternalStoreError(e.to_string()))?;
        Ok(())
    }
}

impl PazckagerStorage for JsonPazckagerStorage {
    fn store_category(&mut self, category: Category) -> StoreResult<()> {
        let category_name = category.category_name.clone();
        self.store.categories.insert(category_name, category);
        self.save_to_file()?;
        Ok(())
    }

    fn get_categories(&self) -> StoreResult<Vec<Category>> {
        Ok(self.store.categories.values().cloned().collect())
    }

    fn category_exists(&self, category_name: &str) -> StoreResult<bool> {
        Ok(self.store.categories.contains_key(category_name))
    }

    fn remove_category(&mut self) -> StoreResult<()> {
        self.store.categories.clear();
        self.save_to_file()?;
        Ok(())
    }

    fn store_package(&mut self, package: PackageData) -> StoreResult<()> {
        let package_name = package.package_name.clone();
        self.store.packages.insert(package_name, package);
        self.save_to_file()?;
        Ok(())
    }

    fn get_packages(&self) -> StoreResult<Vec<PackageData>> {
        Ok(self.store.packages.values().cloned().collect())
    }

    fn package_exists(&self, package_name: &str) -> StoreResult<bool> {
        Ok(self.store.packages.contains_key(package_name))
    }

    fn get_package(&self, package_name: &str) -> StoreResult<PackageData> {
        self.store
            .packages
            .get(package_name)
            .cloned()
            .ok_or(Error::InternalStoreError("Package not found".to_string()))
    }

    fn get_packages_by_category(&self, category_name: &str) -> StoreResult<Vec<PackageData>> {
        Ok(self
            .store
            .packages
            .values()
            .filter(|p| p.category_name == category_name)
            .cloned()
            .collect())
    }

    fn remove_package(&mut self) -> StoreResult<()> {
        self.store.packages.clear();
        self.save_to_file()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pazckager_core::models::InstallationTools;

    use super::*;
    use std::fs;

    #[test]
    fn test_json_storage() -> StoreResult<()> {
        let temp_file = "test_store.json";
        let mut storage = JsonPazckagerStorage::new(temp_file)?;

        let category = Category {
            category_name: "test".to_string(),
            additional_info: Some("info".to_string()),
        };
        storage.store_category(category.clone())?;
        assert!(storage.category_exists("test")?);
        let categories = storage.get_categories()?;
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0], category);

        let package = PackageData {
            package_name: "test_pkg".to_string(),
            instalation_tool: InstallationTools::Pacman,
            category_name: "test".to_string(),
        };
        storage.store_package(package.clone())?;
        assert!(storage.package_exists("test_pkg")?);
        let packages = storage.get_packages()?;
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0], package);

        fs::remove_file(temp_file).unwrap();
        Ok(())
    }
}

use std::{collections::HashMap, str::FromStr};

use crate::err::Error;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    PazckagerCore,
    traits::{InstallationTool, PazckagerStorage},
};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub enum InstallationTools {
    Pacman,
    Yay,
}

impl FromStr for InstallationTools {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "pacman" => InstallationTools::Pacman,
            "yay" => InstallationTools::Pacman,
            _ => Err("Tool is not supported")?,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Category {
    pub category_name: String,
    pub additional_info: Option<String>,
}

#[derive(Serialize, Deserialize, Partial, Clone, PartialEq, Eq, Debug)]
#[partial("RawPackageData", omit(installation_tool))]
pub struct PackageData {
    pub package_name: String,
    pub instalation_tool: InstallationTools,
    pub category_name: String,
}

pub struct PazckagerCoreBuilder<T> {
    store: T,
    package_installers: HashMap<InstallationTools, Box<dyn InstallationTool>>,
}

impl<T: PazckagerStorage> PazckagerCoreBuilder<T> {
    pub fn new(store: T) -> Self {
        Self {
            store,
            package_installers: HashMap::new(),
        }
    }

    pub fn with_installer(mut self, installation_tool: impl InstallationTool + 'static) -> Self {
        self.package_installers
            .insert(installation_tool.get_type(), Box::new(installation_tool));

        self
    }

    pub fn build(self) -> Result<PazckagerCore<T>, BuilderError> {
        if self.package_installers.is_empty() {
            Err(BuilderError::NoPackageInstaller)
        } else {
            Ok(PazckagerCore::new(self.store, self.package_installers)?)
        }
    }
}

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("Colud not find a package installer")]
    NoPackageInstaller,
    #[error("Pazckager core error: {0}")]
    PazckagerCore(#[from] Error),
}

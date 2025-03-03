use pazckager_core::models::{InstallationTools, RawPackageData};
use pazckager_core::traits::{InstallationTool, ToolError, ToolResult};
use std::process::Command;

pub struct PacmanInstaller {
    sudo: bool,
}

impl PacmanInstaller {
    pub fn new(sudo: bool) -> Self {
        Self { sudo }
    }
}

impl InstallationTool for PacmanInstaller {
    fn get_type(&self) -> InstallationTools {
        InstallationTools::Pacman
    }

    fn install_package(&mut self, package_name: &str) -> ToolResult<()> {
        let mut command = if self.sudo {
            let mut cmd = Command::new("sudo");
            cmd.arg("pacman");
            cmd
        } else {
            Command::new("pacman")
        };

        let status = command
            .args(["-S", package_name, "--noconfirm"])
            .status()
            .map_err(|e| {
                ToolError::InstallingPackage(format!("Failed to execute pacman: {}", e))
            })?;

        if status.success() {
            Ok(())
        } else {
            Err(ToolError::InstallingPackage(format!(
                "Pacman failed to install package {} with exit code: {}",
                package_name,
                status.code().unwrap_or(-1)
            )))
        }
    }

    fn delete_package(&mut self, package_name: &str) -> ToolResult<()> {
        let mut command = if self.sudo {
            let mut cmd = Command::new("sudo");
            cmd.arg("pacman");
            cmd
        } else {
            Command::new("pacman")
        };

        let status = command
            .args(["-Rns", package_name, "--noconfirm"])
            .status()
            .map_err(|e| ToolError::DeletingPackage(format!("Failed to execute pacman: {}", e)))?;

        if status.success() {
            Ok(())
        } else {
            Err(ToolError::DeletingPackage(format!(
                "Pacman failed to remove package {} with exit code: {}",
                package_name,
                status.code().unwrap_or(-1)
            )))
        }
    }

    fn update_package(&mut self, package_name: &str) -> ToolResult<()> {
        let mut command = if self.sudo {
            let mut cmd = Command::new("sudo");
            cmd.arg("pacman");
            cmd
        } else {
            Command::new("pacman")
        };

        let status = command
            .args(["-Sy", package_name, "--noconfirm"])
            .status()
            .map_err(|e| ToolError::UpdatingPackage(format!("Failed to execute pacman: {}", e)))?;

        if status.success() {
            Ok(())
        } else {
            Err(ToolError::UpdatingPackage(format!(
                "Pacman failed to update package {} with exit code: {}",
                package_name,
                status.code().unwrap_or(-1)
            )))
        }
    }

    fn get_packages(&self) -> Vec<RawPackageData> {
        let mut command = if self.sudo {
            let mut cmd = Command::new("sudo");
            cmd.arg("pacman");
            cmd
        } else {
            Command::new("pacman")
        };

        match command.args(["-Qe"]).output() {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .map(|line| {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        RawPackageData {
                            package_name: parts.first().unwrap_or(&"").to_string(),
                            category_name: String::new(),
                            installation_tool: InstallationTools::Pacman,
                        }
                    })
                    .collect()
            }
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacman_installer_creation() {
        let installer = PacmanInstaller::new(true);
        assert_eq!(installer.get_type(), InstallationTools::Pacman);
    }

    #[test]
    #[ignore]
    fn test_get_packages() {
        let installer = PacmanInstaller::new(false);
        let packages = installer.get_packages();
        assert!(!packages.is_empty(), "Should return some packages");
    }
}

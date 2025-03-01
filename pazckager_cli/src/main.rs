use clap::{Args, Parser, Subcommand};
use std::collections::HashMap;

// Assuming these are your existing imports
use pazckager_core::{models::PazckagerCoreBuilder, *};
use models::{Category, InstallationTools, PackageData};
use traits::{InstallationTool, PazckagerStorage};
use err::{Error, Result};

// CLI structure definition
#[derive(Parser)]
#[command(name = "pazckager")]
#[command(about = "Package management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a new package
    AddPackage(AddPackageArgs),
    /// Deletes an existing package
    DeletePackage(DeletePackageArgs),
    /// Updates an existing package
    UpdatePackage(UpdatePackageArgs),
    /// Lists all packages
    ListPackages,
    /// Lists all categories
    ListCategories,
    /// Lists packages in a specific category
    ListCategoryPackages(ListCategoryPackagesArgs),
}

#[derive(Args)]
struct AddPackageArgs {
    /// Name of the package to add
    #[arg(short, long)]
    package_name: String,
    /// Installation tool to use (optional)
    #[arg(short, long)]
    tool: Option<InstallationTools>,
    /// Category for the package (optional)
    #[arg(short, long)]
    category: Option<String>,
}

#[derive(Args)]
struct DeletePackageArgs {
    /// Name of the package to delete
    #[arg(short, long)]
    package_name: String,
}

#[derive(Args)]
struct UpdatePackageArgs {
    /// Name of the package to update
    #[arg(short, long)]
    package_name: String,
}

#[derive(Args)]
struct ListCategoryPackagesArgs {
    /// Name of the category to list packages from
    #[arg(short, long)]
    category_name: String,
}

// Main function with CLI integration
fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize your storage and installers (you'll need to replace this with your actual implementation)
    let store = local_db::JsonPazckagerStorage::new("~/.local/share/pazckager_store.json");
    let pacman = ;
    // Add your installers to the HashMap here
    // package_installers.insert(InstallationTools::SomeTool, Box::new(SomeInstaller::new()));

    // Create PazckagerCore instance
    let mut core = PazckagerCoreBuilder::new(store).unwrap();

    // Handle commands
    match cli.command {
        Commands::AddPackage(args) => {
            core.add_package(args.package_name, args.tool, args.category)?;
            println!("Package added successfully");
        }
        Commands::DeletePackage(args) => {
            core.delete_package(args.package_name)?;
            println!("Package deleted successfully");
        }
        Commands::UpdatePackage(args) => {
            core.update_package(args.package_name)?;
            println!("Package updated successfully");
        }
        Commands::ListPackages => {
            let packages = core.get_packages()?;
            println!("Packages:");
            for package in packages {
                println!(
                    "- {} (Tool: {:?}, Category: {})",
                    package.package_name, package.instalation_tool, package.category_name
                );
            }
        }
        Commands::ListCategories => {
            let categories = core.get_categories()?;
            println!("Categories:");
            for category in categories {
                println!("- {}", category.category_name);
                if let Some(info) = category.additional_info {
                    println!("  Additional Info: {}", info);
                }
            }
        }
        Commands::ListCategoryPackages(args) => {
            let packages = core.get_package_by_category(args.category_name)?;
            println!("Packages in category:");
            for package in packages {
                println!(
                    "- {} (Tool: {:?})",
                    package.package_name, package.instalation_tool
                );
            }
        }
    }

    Ok(())
}

// Add this to your Cargo.toml dependencies:
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }

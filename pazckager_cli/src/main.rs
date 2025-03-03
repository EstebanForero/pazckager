use clap::{Args, Parser, Subcommand};

// Assuming these are your existing imports
use err::Result;
use models::InstallationTools;
use pacman_bindings::PermissionMethod;
use pazckager_core::{models::PazckagerCoreBuilder, *};
use pazckager_json_storage::JsonPazckagerStorage;

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
    /// install a new package
    InstallPackage(InstallPackageArgs),
    /// Deletes an existing package
    DeletePackage(DeletePackageArgs),
    /// Unistall an existing package
    UninstallPackage(DeletePackageArgs),
    /// Updates an existing package
    UpdatePackage(UpdatePackageArgs),
    /// Lists all packages
    ListPackages,
    /// Lists all categories
    ListCategories,
    /// Lists packages in a specific category
    ListCategoryPackages(ListCategoryPackagesArgs),
    /// Sync packages
    SyncPackages,
    /// Install all packages in a category
    InstallCategory(InstallCategoryArgs),
    /// Add a new category
    AddCategory(AddCategoryArgs),
    /// Uninstall all packages in a category
    UninstallCategory(UninstallCategoryArgs),
    /// Delete a category
    DeleteCategory(DeleteCategoryArgs),
    /// Change package category
    ChangePackageCategory(ChangePackageCategoryArgs),
    // Get the info of a category
    GetCategory(GetCategoryArgs),
}

#[derive(Args)]
struct GetCategoryArgs {
    /// Name of the package to add
    #[arg(short, long)]
    category_name: String,
}

#[derive(Args)]
struct InstallPackageArgs {
    /// Name of the package to add
    #[arg(short, long)]
    package_name: String,
}

#[derive(Args)]
struct AddPackageArgs {
    /// Name of the package to add
    #[arg(short, long)]
    package_name: String,
    /// Installation tool to use, (pacman)
    #[arg(short, long)]
    tool: InstallationTools,
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

#[derive(Args)]
struct InstallCategoryArgs {
    /// Name of the category to install
    #[arg(short, long)]
    category_name: String,
}

#[derive(Args)]
struct AddCategoryArgs {
    /// Name of the category to add
    #[arg(short, long)]
    category_name: String,
    /// Additional info about the category
    #[arg(short, long)]
    additional_info: Option<String>,
}

#[derive(Args)]
struct UninstallCategoryArgs {
    /// Name of the category to uninstall
    #[arg(short, long)]
    category_name: String,
}

#[derive(Args)]
struct DeleteCategoryArgs {
    /// Name of the category to delete
    #[arg(short, long)]
    category_name: String,
}

#[derive(Args)]
struct ChangePackageCategoryArgs {
    /// Name of the package to change category
    #[arg(short, long)]
    package_name: String,
    /// New category name
    #[arg(short, long)]
    new_category: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let store = JsonPazckagerStorage::new("~/.local/share/pazckager_store.json").unwrap();
    let pacman = pacman_bindings::PacmanInstaller::new(PermissionMethod::Sudo);

    let mut core = PazckagerCoreBuilder::new(store)
        .with_installer(pacman)
        .build()
        .unwrap();

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
                    "- {} (Tool: {:?}, Category: {}, installed: {})",
                    package.package_name,
                    package.installation_tool,
                    package.category_name,
                    package.installed
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
                    "- {} (Tool: {:?}, installed: {})",
                    package.package_name, package.installation_tool, package.installed
                );
            }
        }
        Commands::SyncPackages => {
            println!("Executng sync packages");
            core.sync_packages()?;
            println!("Packages succesfully sync");
        }
        Commands::InstallPackage(args) => {
            if let Err(err) = core.install_package(args.package_name) {
                println!("Error installing package: {err}");
            }
            println!("Package uninstalled succesfully");
        }
        Commands::UninstallPackage(args) => {
            if let Err(err) = core.uninstall_package(args.package_name) {
                println!("Error installing package: {err}");
            }
            println!("Package uninstalled succesfully");
        }
        Commands::InstallCategory(args) => {
            core.install_category(args.category_name)?;
            println!("Category installed successfully");
        }
        Commands::AddCategory(args) => {
            core.add_category(args.category_name, args.additional_info)?;
            println!("Category added successfully");
        }
        Commands::UninstallCategory(args) => {
            core.uninstall_category(args.category_name)?;
            println!("Category uninstalled successfully");
        }
        Commands::DeleteCategory(args) => {
            core.delete_category(args.category_name)?;
            println!("Category deleted successfully");
        }
        Commands::ChangePackageCategory(args) => {
            core.change_package_category(args.new_category, args.package_name)?;
            println!("Package category changed successfully");
        }
        Commands::GetCategory(args) => {
            let category = core.get_category(args.category_name)?;
            println!("Category name: {}", category.category_name);
            println!(
                "Category info: {}",
                category.additional_info.unwrap_or(String::new())
            );
        }
    }

    Ok(())
}

// Add this to your Cargo.toml dependencies:
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }

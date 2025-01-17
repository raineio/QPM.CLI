use clap::Args;
use owo_colors::OwoColorize;

use crate::{
    commands::Command,
    repository::{multi::MultiDependencyRepository, Repository},
};

#[derive(Args, Debug, Clone)]
pub struct PackageCommand {
    pub package: String,
    #[clap(short, long)]
    pub latest: bool,
}

impl Command for PackageCommand {
    fn execute(self) -> color_eyre::Result<()> {
        let versions =
            MultiDependencyRepository::useful_default_new()?.get_package_versions(&self.package)?;
        if self.latest {
            println!(
                "The latest version for package {} is {}",
                self.package.bright_red(),
                versions
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .version
                    .to_string()
                    .bright_green()
            );

            return Ok(())
        }

        match &versions {
            Some(package_versions) => {
                println!(
                    "Package {} has {} versions on qpackages.com:",
                    self.package.bright_red(),
                    versions.as_ref().unwrap().len().bright_yellow()
                );
                for package_version in package_versions.iter().rev() {
                    println!(" - {}", package_version.version.to_string().bright_green());
                }
            }
            _ => {
                println!(
                    "Package {} either did not exist or has no versions on qpackages.com",
                    self.package.bright_red()
                );
            }
        }
        Ok(())
    }
}

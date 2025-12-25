use std::path::Path;
use std::process::Command;

use thiserror::Error;

use crate::project::PackageManagerKind;

#[derive(Debug, Clone)]
pub struct InstallPlan {
    pub manager: PackageManagerKind,
    pub packages: Vec<String>,
    pub dev: bool,
}

#[derive(Debug, Error)]
pub enum PackageManagerError {
    #[error("package manager not supported: {0:?}")]
    Unsupported(PackageManagerKind),
    #[error("failed to run package manager: {0}")]
    Execution(String),
}

impl InstallPlan {
    pub fn new(manager: PackageManagerKind) -> Self {
        Self {
            manager,
            packages: Vec::new(),
            dev: false,
        }
    }

    pub fn add_packages<I, S>(&mut self, packages: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for pkg in packages {
            self.packages.push(pkg.into());
        }
    }

    pub fn dev(mut self, value: bool) -> Self {
        self.dev = value;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    pub fn run(&self, cwd: &Path) -> Result<(), PackageManagerError> {
        if self.packages.is_empty() {
            return Ok(());
        }

        let mut cmd = match self.manager {
            PackageManagerKind::Npm => {
                let mut command = Command::new("npm");
                command.arg("install");
                if self.dev {
                    command.arg("--save-dev");
                }
                command
            }
            PackageManagerKind::Pnpm => {
                let mut command = Command::new("pnpm");
                command.arg("add");
                if self.dev {
                    command.arg("-D");
                }
                command
            }
            PackageManagerKind::Yarn => {
                let mut command = Command::new("yarn");
                command.arg("add");
                if self.dev {
                    command.arg("-D");
                }
                command
            }
            PackageManagerKind::Bun => {
                let mut command = Command::new("bun");
                command.arg("add");
                if self.dev {
                    command.arg("-d");
                }
                command
            }
            PackageManagerKind::Unknown => {
                return Err(PackageManagerError::Unsupported(self.manager));
            }
        };

        cmd.current_dir(cwd);
        for pkg in &self.packages {
            cmd.arg(pkg);
        }

        let status = cmd
            .status()
            .map_err(|err| PackageManagerError::Execution(err.to_string()))?;
        if status.success() {
            Ok(())
        } else {
            Err(PackageManagerError::Execution(format!(
                "command exited with status {status}"
            )))
        }
    }
}

use std::ffi::OsString;
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

        let mut cmd = self.build_command();
        cmd.current_dir(cwd);

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

    pub fn build_command(&self) -> Command {
        let mut cmd = match self.manager {
            PackageManagerKind::Npm => {
                let mut command = Command::new(pkg_command("npm", true));
                command.arg("install");
                if self.dev {
                    command.arg("--save-dev");
                }
                command
            }
            PackageManagerKind::Pnpm => {
                let mut command = Command::new(pkg_command("pnpm", true));
                command.arg("add");
                if self.dev {
                    command.arg("-D");
                }
                command
            }
            PackageManagerKind::Yarn => {
                let mut command = Command::new(pkg_command("yarn", true));
                command.arg("add");
                if self.dev {
                    command.arg("-D");
                }
                command
            }
            PackageManagerKind::Bun => {
                let mut command = Command::new(pkg_command("bun", false));
                command.arg("add");
                if self.dev {
                    command.arg("-d");
                }
                command
            }
            PackageManagerKind::Unknown => {
                let mut c = Command::new("echo");
                c.arg("unknown-manager");
                c
            }
        };

        for pkg in &self.packages {
            cmd.arg(pkg);
        }
        cmd
    }
}

fn pkg_command(base: &str, needs_cmd: bool) -> OsString {
    #[cfg(windows)]
    {
        if needs_cmd {
            let mut name = OsString::from(base);
            name.push(".cmd");
            name
        } else {
            OsString::from(base)
        }
    }
    #[cfg(not(windows))]
    {
        let _ = needs_cmd;
        OsString::from(base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_plan_state_mutations() {
        let mut plan = InstallPlan::new(PackageManagerKind::Npm);
        assert!(plan.is_empty());
        assert!(!plan.dev);
        assert_eq!(plan.manager, PackageManagerKind::Npm);

        plan.add_packages(vec!["react", "react-dom"]);
        assert!(!plan.is_empty());
        assert_eq!(plan.packages.len(), 2);
        assert_eq!(plan.packages[0], "react");

        plan = plan.dev(true);
        assert!(plan.dev);
    }

    #[test]
    fn install_plan_handles_different_managers() {
        let plan = InstallPlan::new(PackageManagerKind::Pnpm);
        assert_eq!(plan.manager, PackageManagerKind::Pnpm);

        let plan = InstallPlan::new(PackageManagerKind::Yarn);
        assert_eq!(plan.manager, PackageManagerKind::Yarn);

        let plan = InstallPlan::new(PackageManagerKind::Bun);
        assert_eq!(plan.manager, PackageManagerKind::Bun);
    }

    #[test]
    fn build_command_generates_correct_args() {
        let mut plan = InstallPlan::new(PackageManagerKind::Npm);
        plan.add_packages(vec!["pkg-a"]);
        let cmd = plan.build_command();
        let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
        assert!(args.contains(&std::ffi::OsStr::new("install")));
        assert!(args.contains(&std::ffi::OsStr::new("pkg-a")));

        let mut plan = InstallPlan::new(PackageManagerKind::Pnpm).dev(true);
        plan.add_packages(vec!["pkg-b"]);
        let cmd = plan.build_command();
        let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
        assert!(args.contains(&std::ffi::OsStr::new("add")));
        assert!(args.contains(&std::ffi::OsStr::new("-D")));
        assert!(args.contains(&std::ffi::OsStr::new("pkg-b")));
    }
}

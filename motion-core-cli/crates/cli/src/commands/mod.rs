pub mod add;
pub mod cache;
pub mod init;
pub mod list;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandOutcome {
    Completed,
    NoOp,
}

pub type CommandResult = Result<CommandOutcome>;

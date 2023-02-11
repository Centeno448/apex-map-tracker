use crate::commands::error::CommandError;

pub type CommandResult<T> = std::result::Result<T, CommandError>;

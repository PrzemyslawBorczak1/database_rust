pub mod create_command;
pub mod insert_command;



pub use create_command::Create;
pub use insert_command::Insert;

use crate::errors::ExecutionResult;

pub trait Command {
    fn execute(self) -> ExecutionResult<()>;
}


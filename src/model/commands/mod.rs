pub mod create_command;
pub mod insert_command;
pub mod delete_command;
pub mod read_command;


pub use create_command::Create;
pub use insert_command::Insert;
pub use delete_command::Delete;
pub use read_command::Read;

use crate::errors::ExecutionResult;

pub trait Command {
    fn execute(self) -> ExecutionResult<()>;
}


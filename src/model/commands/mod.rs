pub mod create_command;
pub mod delete_command;
pub mod insert_command;
pub mod read_command;
pub mod save_command;
pub mod select_command;

pub use create_command::Create;
pub use delete_command::Delete;
pub use insert_command::Insert;
pub use read_command::Read;
pub use save_command::Save;
pub use select_command::Select;

use crate::errors::ExecutionResult;

pub trait Command {
    fn execute(self) -> ExecutionResult<()>;
}

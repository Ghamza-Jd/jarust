pub use crate::error::JaError;
pub use crate::jahandle::JaHandle;
pub use crate::japlugin::Attach;
pub use crate::japlugin::PluginTask;
pub use crate::jasession::JaSession;
pub use jarust_interface::japrotocol::JaResponse;

pub type JaResult<T> = core::result::Result<T, JaError>;

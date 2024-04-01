pub use crate::error::JaError;
pub use crate::jaconfig::BUFFER_SIZE;
pub use crate::jaconnection::JaResponseStream;
pub use crate::jahandle::JaHandle;
pub use crate::japlugin::Attach;
pub use crate::japlugin::PluginTask;
pub use crate::japrotocol::JaResponse;
pub use crate::jasession::JaSession;
pub use crate::jatask;
pub use crate::jatask::AbortHandle;
pub use crate::transport::trans::MessageStream;

pub type JaResult<T> = core::result::Result<T, JaError>;

use crate::error::JaTransportError;

pub type JaTransportResult<T> = core::result::Result<T, JaTransportError>;

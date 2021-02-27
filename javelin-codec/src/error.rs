#[cfg(feature = "mpegts")]
use crate::mpegts::TsError;
use {
    crate::{aac::AacError, avc::AvcError, flv::FlvError},
    thiserror::Error,
};

#[derive(Error, Debug)]
pub enum CodecError {
    #[error(transparent)]
    AvcError(#[from] AvcError),

    #[error(transparent)]
    AacError(#[from] AacError),

    #[error(transparent)]
    FlvError(#[from] FlvError),

    #[cfg(feature = "mpegts")]
    #[error(transparent)]
    TsError(#[from] TsError),
}

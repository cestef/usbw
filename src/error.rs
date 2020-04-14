use driver_async::error::IOError;
pub struct Error(pub IOError);
impl From<rusb::Error> for Error {
    fn from(e: rusb::Error) -> Self {
        Error(match e {
            rusb::Error::Success => unreachable!("success passed as error type"),
            rusb::Error::Io => IOError::Other,
            rusb::Error::InvalidParam => IOError::InvalidArgument,
            rusb::Error::Access => IOError::AccessDenied,
            rusb::Error::NoDevice => IOError::NotConnected,
            rusb::Error::NotFound => IOError::NotFound,
            rusb::Error::Busy => IOError::Refused,
            rusb::Error::Timeout => IOError::TimedOut,
            rusb::Error::Overflow => IOError::Overflow,
            rusb::Error::Pipe => IOError::Pipe,
            rusb::Error::Interrupted => IOError::Interrupted,
            rusb::Error::NoMem => IOError::OutOfMemory,
            rusb::Error::NotSupported => IOError::NotImplemented,
            rusb::Error::Other => IOError::Other,
        })
    }
}

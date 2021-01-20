//! # Error handling
//!
//! The error handling for mqtt-helper

/// A specialized result type.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    /// SetupWatchdog error.
    SetupWatchDog,

    /// WatchdogHeartbeat error.
    SendHeartbeat,

    /// TopicNotFound error.
    TopicNotFound,

    /// ClientId error.
    ClientId,

    /// MqttHelperSetUserPass error.
    MqttHelperSetUserPass,

    /// MqttHelperSetupTls error.
    MqttHelperSetupTls,

    /// MqttHelperConnect error.
    MqttHelperConnect,

    /// MqttHelperReconnect error.
    MqttHelperReconnect,

    /// MqttHelperSocket
    MqttHelperSocket,

    /// MqttRunLoop error.
    MqttHelperRunLoop,

    /// MqttHelperRunLoopStart eror.
    MqttHelperRunLoopStart,

    /// MqttHelperPayload error.
    MqttHelperPayload,

    /// MqttHelperPublish error.
    MqttHelperPublish,

    /// MqttHelperSubscribe error.
    MqttHelperSubscribe,

    /// MqttHelperUnsubscribe error.
    MqttHelperUnsubscribe,

    /// Any boxed error.
    Boxed(Box<dyn std::error::Error>),

    /// I/O error.
    Io(std::io::Error),

    /// ParseInt error.
    ParseInt(std::num::ParseIntError),

    /// MosquittoMqtt error.
    MosquittoMqtt(mosquitto_mqtt::Error),

    /// NulError.
    NulError(std::ffi::NulError),

    /// Other errors.
    Other,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    /// The printed representation of an error kind.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Boxed(error) => write!(f, "{}", error),
            Error::Io(error) => write!(f, "{}", error),
            Error::ParseInt(error) => write!(f, "{}", error),
            Error::MosquittoMqtt(error) => write!(f, "{}", error),
            Error::NulError(error) => write!(f, "{}", error),
            Error::SetupWatchDog => write!(f, "SetupWatchdog error"),
            Error::SendHeartbeat => write!(f, "Watchdog heartbeat error"),
            Error::TopicNotFound => write!(f, "TopicNotFound error"),
            Error::ClientId => write!(f, "ClientId error"),
            Error::MqttHelperSetUserPass => write!(f, "Set User&Password error"),
            Error::MqttHelperSetupTls => write!(f, "SetupTls error"),
            Error::MqttHelperConnect => write!(f, "MqttHelperConnect error"),
            Error::MqttHelperReconnect => write!(f, "MqttHelperReconnect error"),
            Error::MqttHelperSocket => write!(f, "MqttHelperSocket error"),
            Error::MqttHelperRunLoop => write!(f, "MqttHelperRunLoop error"),
            Error::MqttHelperRunLoopStart => write!(f, "MqttHelperRunLoopStart error"),
            Error::MqttHelperPayload => write!(f, "MqttHelperPayload error"),
            Error::MqttHelperPublish => write!(f, "MqttHelperPublish error"),
            Error::MqttHelperSubscribe => write!(f, "MqttHelperSubscribe error"),
            Error::MqttHelperUnsubscribe => write!(f, "MqttHelperUnsubscribe error"),
            Error::Other => write!(f, "Error"),
        }
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    /// A source containing any boxed error.
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Boxed(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<mosquitto_mqtt::Error> for Error {
    fn from(error: mosquitto_mqtt::Error) -> Self {
        Error::MosquittoMqtt(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}

impl From<std::ffi::NulError> for Error {
    /// A source containing a nul error.
    fn from(error: std::ffi::NulError) -> Self {
        Error::NulError(error)
    }
}

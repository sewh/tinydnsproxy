use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    Io(std::io::Error),
    Tls(native_tls::Error),
    TlsHandshake(native_tls::HandshakeError<std::net::TcpStream>),
    Toml(toml::de::Error),
    Curl(curl::Error),
    TextDecodingError(std::string::FromUtf8Error),
    NoDotProviders,
    SyncError,
    LineIsBlank,
    DnsParsing,
    DnsTooManyQuestions,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;

        let message = match &self.kind {
            Io(e) => format!("IO error: {}", e),
            Tls(e) => format!("TLS error: {}", e),
            TlsHandshake(e) => format!("TLS handshake error: {}", e),
            Toml(e) => format!("TOML file error: {}", e),
            Curl(e) => format!("HTTP error: {}", e),
            TextDecodingError(e) => format!("Couldn't decode bytes as UTF-8: {}", e),
            NoDotProviders => {
                "TOML file error: At least one DNS-over-TLS provider must be defined".to_string()
            }
            SyncError => "Unable to update block list".to_string(),
            LineIsBlank => "Block list line has no meaningful content on it".to_string(),
            DnsParsing => "Couldn't parse DNS message".to_string(),
            DnsTooManyQuestions => "Too many questions in DNS request".to_string(),
        };

        write!(f, "{}", message)
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }

    pub fn no_dot_providers() -> Self {
        Self::new(ErrorKind::NoDotProviders)
    }

    pub fn sync_error() -> Self {
        Self::new(ErrorKind::SyncError)
    }

    pub fn line_is_blank() -> Self {
        Self::new(ErrorKind::LineIsBlank)
    }

    pub fn dns_parsing() -> Self {
        Self::new(ErrorKind::DnsParsing)
    }

    pub fn dns_too_many_questions() -> Self {
        Self::new(ErrorKind::DnsTooManyQuestions)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorKind::Io(e))
    }
}

impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Self::new(ErrorKind::Tls(e))
    }
}

impl From<native_tls::HandshakeError<std::net::TcpStream>> for Error {
    fn from(e: native_tls::HandshakeError<std::net::TcpStream>) -> Self {
        Self::new(ErrorKind::TlsHandshake(e))
    }
}

impl From<curl::Error> for Error {
    fn from(e: curl::Error) -> Self {
        Self::new(ErrorKind::Curl(e))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::new(ErrorKind::TextDecodingError(e))
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        Self::sync_error()
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::new(ErrorKind::Toml(e))
    }
}

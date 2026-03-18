use core::fmt;

/// No-std 兼容的错误 trait
pub trait NoStdError: fmt::Debug + fmt::Display {
    fn source(&self) -> Option<&(dyn NoStdError + 'static)> {
        None
    }
}

impl<E> NoStdError for E
where
    E: fmt::Debug + fmt::Display + 'static,
{
    fn source(&self) -> Option<&(dyn NoStdError + 'static)> {
        None
    }
}

/// 基础错误类型
#[derive(Debug)]
pub enum Error {
    /// I/O 错误
    IoError,
    /// 解析错误
    ParseError,
    /// 配置错误
    ConfigError,
    /// 内存错误
    MemoryError,
    /// 网络错误
    NetworkError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError => write!(f, "I/O error"),
            Error::ParseError => write!(f, "Parse error"),
            Error::ConfigError => write!(f, "Configuration error"),
            Error::MemoryError => write!(f, "Memory error"),
            Error::NetworkError => write!(f, "Network error"),
        }
    }
}

impl NoStdError for Error {}

/// 链式错误类型
#[derive(Debug)]
pub struct ChainError {
    kind: ErrorKind,
    source: Option<Box<dyn NoStdError>>,
}

#[derive(Debug)]
pub enum ErrorKind {
    Io,
    Parse,
    Config,
    Memory,
    Network,
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Io => write!(f, "I/O error"),
            ErrorKind::Parse => write!(f, "Parse error"),
            ErrorKind::Config => write!(f, "Configuration error"),
            ErrorKind::Memory => write!(f, "Memory error"),
            ErrorKind::Network => write!(f, "Network error"),
        }
    }
}

impl NoStdError for ChainError {
    fn source(&self) -> Option<&(dyn NoStdError + 'static)> {
        self.source.as_deref()
    }
}

/// 结果类型别名
pub type Result<T> = core::result::Result<T, Box<dyn NoStdError>>;

/// 错误处理宏
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err(Box::new($e));
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !$cond {
            bail!($e);
        }
    };
}

/// 从标准错误转换
#[cfg(feature = "std")]
pub fn from_std_error(error: std::io::Error) -> Box<dyn NoStdError> {
    Box::new(Error::IoError)
}

/// 错误日志记录
#[cfg(feature = "log")]
pub fn log_error(error: &dyn NoStdError) {
    log::error!("{}", error);
}

/// 错误处理助手
pub fn handle_error(result: Result<()>) {
    match result {
        Ok(()) => {}
        Err(e) => {
            #[cfg(feature = "log")]
            log_error(&*e);

            // 在 no-std 环境中，可能需要重启或panic
            loop {} // 无限循环表示严重错误
        }
    }
}
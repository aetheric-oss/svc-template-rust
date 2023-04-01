//! log macro's for REST logging

/// Writes a debug! message to the app::rest logger
#[macro_export]
macro_rules! rest_debug {
    ($($arg:tt)+) => {
        log::debug!(target: "app::rest", $($arg)+)
    };
}

/// Writes an info! message to the app::rest logger
#[macro_export]
macro_rules! rest_info {
    ($($arg:tt)+) => {
        log::info!(target: "app::rest", $($arg)+)
    };
}

/// Writes an warn! message to the app::rest logger
#[macro_export]
macro_rules! rest_warn {
    ($($arg:tt)+) => {
        log::warn!(target: "app::rest", $($arg)+)
    };
}

/// Writes an error! message to the app::rest logger
#[macro_export]
macro_rules! rest_error {
    ($($arg:tt)+) => {
        log::error!(target: "app::rest", $($arg)+)
    };
}

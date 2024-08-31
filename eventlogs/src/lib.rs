pub mod communication;
pub mod event;
pub mod models;

pub extern crate chrono;

#[macro_export]
macro_rules! send {
    ($msg: ident) => {{
        if let Some(com) = $crate::communication::COMMS.get() {
            com.send($msg);
        }
    }};
}

#[macro_export]
macro_rules! log {
    ($level:expr, $msg:expr) => {{
        let msg = $crate::models::message::Message::Log($crate::models::log::Log {
            level: String::from($level),
            message: String::from($msg),
            timestamp: $crate::chrono::Utc::now().to_rfc3339(),
        });

        $crate::send!(msg);
    }};
}

#[macro_export]
macro_rules! trace {
    ($msg:expr) => {
        $crate::log!("TRACE", $msg);
    };
    ($msg:expr, $( $arg:tt )*) => {
        $crate::log!("TRACE", format!($msg, $( $arg )*));
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        $crate::log!("DEBUG", $msg);
    };
    ($msg:expr, $( $arg:tt )*) => {
        $crate::log!("DEBUG", format!($msg, $( $arg )*));
    };
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::log!("INFO", $msg);
    };
    ($msg:expr, $( $arg:tt )*) => {
        $crate::log!("INFO", format!($msg, $( $arg )*));
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::log!("ERROR", $msg);
    };
    ($msg:expr, $( $arg:tt )*) => {
        $crate::log!("ERROR", format!($msg, $( $arg )*));
    };
}

#[macro_export]
macro_rules! agent {
    ($agent:expr) => {{
        let msg = $crate::models::message::Message::Agent($agent);
        $crate::send!(msg);
    }};
}

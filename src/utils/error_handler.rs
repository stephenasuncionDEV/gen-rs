#[macro_export]
macro_rules! throw {
    (target: $target:expr, $($arg:tt)+) => {
        log::log!(target: $target, $crate::Level::Error, $($arg)+);
        std::process::exit(-1);
    };
    ($($arg:tt)+) => {
        log::log!($crate::Level::Error, $($arg)+);
        std::process::exit(-1);
    };
}
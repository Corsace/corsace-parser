#[macro_export]
macro_rules! console_log {

    ($($t:tt)*) => ({
        use crate::log;
        log(&format_args!($($t)*).to_string())})
}

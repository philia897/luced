#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)] // This ensures the code is only included in Debug mode
        {
            println!($($arg)*);
        }
    };
}
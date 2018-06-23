#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use std::fmt::Write;
        let mut print_writer = $crate::PrintWriter;
        let _ = write!(&mut print_writer, $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

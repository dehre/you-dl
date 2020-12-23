#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (println!("{:>7} {}", ($crate::colored::Colorize::blue("Info:")), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (println!("{:>7} {}", ($crate::colored::Colorize::yellow("Warn:")), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! select {
    ($($arg:tt)*) => (println!("{:>7} {}", ($crate::colored::Colorize::bold($crate::colored::Colorize::yellow("Select:"))), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => (println!("{:>7} {}", ($crate::colored::Colorize::bold($crate::colored::Colorize::green("Success:"))), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (println!("{:>7} {}", ($crate::colored::Colorize::bold($crate::colored::Colorize::red("Error:"))), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! failed {
    ($($arg:tt)*) => (println!("{:>7} {}", ($crate::colored::Colorize::bold($crate::colored::Colorize::red("Failed:"))), format_args!($($arg)*)));
}

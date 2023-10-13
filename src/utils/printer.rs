const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

pub fn info_print<T>(message: T)
where
    T: std::fmt::Display,
{
    println!("[{BLUE}INFO{RESET}] {}", message);
}
pub fn err_print<T>(message: T)
where
    T: std::fmt::Display,
{
    eprintln!("[{RED}ERROR{RESET}] {}", message);
}

pub fn warn_print<T>(message: T)
where
    T: std::fmt::Display,
{
    eprintln!("[{YELLOW}WARN{RESET}] {}", message);
}

pub fn success_print<T>(message: T)
where
    T: std::fmt::Display,
{
    eprintln!("[{GREEN}SUCCESS{RESET}] {}", message);
}

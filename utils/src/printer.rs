const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

/// Prints an informational message to the console.
///
/// # Arguments
///
/// * `message` - The message to be printed.
///
/// # Example
///
/// ```rust
/// info_print("This is an informational message");
/// ```
pub fn info_print<T>(message: T)
where
    T: std::fmt::Display,
{
    println!("[{BLUE}INFO{RESET}]  {}", message);
}

/// Prints an error message to the standard error stream.
///
/// # Arguments
///
/// * `message` - The error message to be printed.
///
/// # Examples
///
/// ```
/// err_print("Something went wrong");
/// ```
pub fn err_print<T>(message: T)
where
    T: std::fmt::Display,
{
    eprintln!("[{RED}ERROR{RESET}] {}", message);
}

/// Prints a warning message to the standard error output.
///
/// # Arguments
///
/// * `message` - The message to be printed.
///
/// # Example
///
/// ```
/// warn_print("This is a warning!");
/// ```
pub fn warn_print<T>(message: T)
where
    T: std::fmt::Display,
{
    eprintln!("[{YELLOW}WARN{RESET}]  {}", message);
}

/// Prints a success message to stderr.
///
/// # Arguments
///
/// * `message` - The message to print.
///
/// # Example
///
/// ```
/// success_print("Operation completed successfully");
/// ```
pub fn success_print<T>(message: T)
where
    T: std::fmt::Display,
{
    eprintln!("[{GREEN}SUCCESS{RESET}] {}", message);
}

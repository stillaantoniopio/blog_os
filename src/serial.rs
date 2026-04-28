use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};

lazy_static! {
    /// A global instance of the first serial interface (COM1).
    ///
    /// We wrap it in a `Mutex` to ensure "thread safety." Even though we don't
    /// have threads yet, this prevents multiple parts of the kernel from
    /// trying to write to the hardware at the exact same time.
    pub static ref SERIAL1: Mutex<Uart16550Tty<PioBackend>> = Mutex::new(unsafe {

        // 0x3F8 is the standard I/O port address for COM1 on x86 systems.
        // This is where the hardware "lives" in the CPU's I/O space.
        let uart = Uart16550Tty::new_port(0x3F8, Config::default())
            .expect("failed to initialize UART");

        uart
    });
}

/// A helper function to make using the serial port as easy as `println!`.
/// This is used by the `_print` function below.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host console via the serial port.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host console via the serial port, adding a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

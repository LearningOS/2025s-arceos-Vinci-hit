//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    // 无参数版本
    () => {
        $crate::print!("\x1b[0m\n")  // 重置颜色并换行
    };
    
    ($msg:expr) => {
        if $msg.starts_with("[WithColor]: ") {
            // 红色显示带标记的文本
            $crate::io::__print_impl(format_args!(
                "\x1b[31m{}\x1b[0m\n",  // 31m=红色，0m=重置
                $msg
            ));
        } else {
            // 普通显示
            $crate::io::__print_impl(format_args!("{}\n", $msg));
        }
    };
    
    // 普通版本(保持默认颜色)
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!(
            "{}\n",
            format_args!($($arg)*)
        ));
    };
}
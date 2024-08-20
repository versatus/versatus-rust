pub mod builders;
pub mod cli;

#[macro_export]
macro_rules! cargo {
    ($($arg:expr),*) => {{
        let mut command = std::process::Command::new("cargo");
        let args = [$(<dyn AsRef<std::ffi::OsStr>>::as_ref($arg),)*];
        command.args(args);
        match command.output() {
            std::result::Result::Ok(o) => Ok(o),
            Err(e) => {
                use std::fmt::Write;
                let mut args_str = String::with_capacity(args.len());
                args.iter()
                    .for_each(|arg| write!(
                        args_str, "{} ", arg.to_str().expect("failed to convert OsStr to str")
                    ).expect("failed to write cargo args into string buffer."));
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "cargo command failed: cargo {}\nError: {e:?}",
                        args_str
                    ),
                ))
            }
        }
    }};
}

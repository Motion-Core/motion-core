use std::fmt::Arguments;

use crate::style::{brand, danger, warning};

pub trait Reporter {
    fn info(&self, message: Arguments<'_>);
    fn warn(&self, message: Arguments<'_>);
    fn error(&self, message: Arguments<'_>);
    fn blank(&self);
}

#[derive(Default)]
pub struct ConsoleReporter;

impl ConsoleReporter {
    pub fn new() -> Self {
        Self
    }

    fn format(args: Arguments<'_>) -> String {
        format!("{args}")
    }
}

impl Reporter for ConsoleReporter {
    fn info(&self, message: Arguments<'_>) {
        let text = Self::format(message);
        println!("{} {}", brand("›"), text);
    }

    fn warn(&self, message: Arguments<'_>) {
        let text = Self::format(message);
        println!("{} {}", warning("!"), warning(&text));
    }

    fn error(&self, message: Arguments<'_>) {
        let text = Self::format(message);
        eprintln!("{} {}", danger("✖"), danger(&text));
    }

    fn blank(&self) {
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestReporter {
        infos: std::sync::Mutex<Vec<String>>,
    }

    impl TestReporter {
        fn new() -> Self {
            Self {
                infos: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    impl Reporter for TestReporter {
        fn info(&self, message: Arguments<'_>) {
            self.infos.lock().unwrap().push(format!("{message}"));
        }

        fn warn(&self, _message: Arguments<'_>) {}
        fn error(&self, _message: Arguments<'_>) {}
        fn blank(&self) {
            self.infos.lock().unwrap().push(String::new());
        }
    }

    #[test]
    fn stores_messages() {
        let reporter = TestReporter::new();
        reporter.info(format_args!("hello {}", 42));
        reporter.warn(format_args!("warn message"));
        reporter.error(format_args!("error message"));
        reporter.blank();
        assert_eq!(reporter.infos.lock().unwrap().len(), 2);
    }
}

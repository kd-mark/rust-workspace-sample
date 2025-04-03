use std::any::type_name;

use crate::helpers::date_formater::DateFormater;

pub trait Logger: Send + Sync {
    fn log(&self, message: &str);
    fn error(&self, message: &str);
    fn warn(&self, message: &str);
    fn debug(&self, message: &str);
}

#[derive(Debug, Clone)]
pub struct DefaultLogger<'a> {
    module_name: &'a str,
}

impl<'a> DefaultLogger<'a> {
    pub fn new<T>() -> Self {
        let type_name = type_name::<T>();
        let a = type_name.split("::");
        Self {
            module_name: a.last().unwrap_or(type_name),
        }
    }
}

impl<'a> Logger for DefaultLogger<'a> {
    fn log(&self, message: &str) {
        println!(
            "\x1b[32m[INFO] {} [{}] - {}",
            DateFormater::datetime(),
            self.module_name,
            message
        );
    }

    fn error(&self, message: &str) {
        eprintln!(
            "\x1b[31m[ERROR] {} [{}] - {}",
            DateFormater::datetime(),
            self.module_name,
            message
        );
    }

    fn warn(&self, message: &str) {
        println!(
            "\x1b[33m[WARN] {} [{}] - {}",
            DateFormater::datetime(),
            self.module_name,
            message
        );
    }

    fn debug(&self, message: &str) {
        println!(
            "\x1b[34m[DEBUG] {} [{}] - {}",
            DateFormater::datetime(),
            self.module_name,
            message
        );
    }
}

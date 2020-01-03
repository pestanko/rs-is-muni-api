use sxd_document::{parser, Package};
use sxd_xpath::{evaluate_xpath, Value};

use crate::errors::IsApiErrors;

pub trait Creatable {
    fn new(package: &Package) -> Result<dyn Creatable>;
    fn parse(body: &str) -> Result<dyn Creatable>;
}

#[derive(Debug, Clone)]
struct CourseInfo {

}

impl <T> Creatable for T where T: Creatable {
    fn parse(body: &str) -> Result<T, IsApiErrors> {
        match parser::parse(body) {
            Ok(package) => T::new(package),
            Err(err) => IsApiErrors::Parse(err)
        }
    }
}

impl Creatable for CourseInfo {
    fn new(package: &Package) {
        
    }
}




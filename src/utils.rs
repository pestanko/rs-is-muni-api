use sxd_document::{dom, parser};
use sxd_xpath::{Value, evaluate_xpath};
use chrono::{DateTime, NaiveDate};
use crate::errors::IsApiErrors;

pub struct DocumentParser<'t> {
    document: &'t dom::Document<'t>,
}

impl DocumentParser {
    pub fn from_document(doc: &dom::Document) -> DocumentParser {
        DocumentParser {
            document: doc,
        }
    }

    pub fn from_string(string: &str) -> Result<DocumentParser, IsApiErrors> {
        match parser::parse(body) {
            Ok(pkg) => T::new(&package.as_document()),
            Err(err) => IsApiErrors::Parse(err)
        }
    }

    pub fn document(&self) -> &dom::Document {
        self.document
    }

    pub fn optional(&self, xpath: &str) -> Option<Value> {
        match evaluate_xpath(self.document, xpath) {
            Ok(value) => Some(value),
            Err(err) => {
                log::warning!("Unable to parse xpath {}: {?:}", xpath, err);
                Option::None
            }
        }
    }

    pub fn value(&self, xpath: &str) -> Value {
        get_optional(self.document, xpath)
            .expect(&format!("Unable to get an value for the XPath: {}", xpath))
    }

    pub fn to_string(&self, xpath: &str) -> String {
        self.value(xpath).into_string()
    }

    pub fn to_u32(&self, xpath: &str) -> u32 {
        self.to_string(xpath).parse::<u32>().unwrap()
    }

    pub fn to_u16(&self, xpath: &str) -> u16 {
        self.to_string(xpath).parse::<u16>().unwrap()
    }

    pub fn to_bool(&self, xpath: &str) -> bool {
        match self.to_string(xpath) {
            String::from("ano") => true,
            _ => false
        }
    }

    pub fn to_datetime(&self, xpath: &str) -> chrono::DateTime<chrono::Local> {
        let full = self.to_string(xpath);
        let year = full[0..4].parse::<u32>().unwrap();
        let month = full[4..6].parse::<u32>().unwrap();
        let day = full[6..8].parse::<u32>().unwrap();
        let hour = full[8..10].parse::<u32>().unwrap();
        let min = full[10..12].parse::<u32>().unwrap();
        let sec = full[12..14].parse::<u32>().unwrap();

        let naive = NaiveDate::from_ymd(year as i32, month, day).and_hms(hour, min, sec);
        chrono::DateTime::from(naive)
    }
}
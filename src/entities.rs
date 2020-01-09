use sxd_document::{parser, Package, dom};
use sxd_xpath::{evaluate_xpath, Value};
use chrono::prelude::*;

use crate::errors::IsApiErrors;
use crate::utils;
use sxd_document::parser::parse;
use crate::utils::DocumentParser;

pub trait Creatable {
    fn new(doc: &dom::Document) -> Result<dyn Creatable, IsApiErrors>;
}

pub trait Parseable {
    fn parse(body: &str) -> Result<dyn Parseable, IsApiErrors>;
}

#[derive(Debug, Clone)]
pub struct Seminar {
    limit_id: u32,
    limit_shareable: bool,
    max_students: u16,
    multiple_enrollment: bool,
    cancel_to: DateTime<Local>,
    enroll_to: DateTime<Local>,
    enroll_from: DateTime<Local>,
    code: String,
    students_in_seminary: u16,
    note: String,
    id: u32,
    updated_at: DateTime<Local>,
    updated_by: u32,
    with_inactive: bool
}

#[derive(Debug, Clone)]
pub struct Teacher {

}


#[derive(Debug, Clone)]
pub struct CourseInfo {
    pub faculty_id: u32,
    pub faculty_shortcut: String,
    pub course_code: String,
    pub course_name: String,
    pub course_short_name: String,
    pub course_short_name_en: String,
    pub course_name_en: String,
    pub course_id: u32,
    pub period_id: u32,
    pub period_name: String,
    pub period_name_en: String,
    pub period_url_shortcut: String,
    pub period_url_shortcut_en: String,
    pub enrolled_students: u16,
    pub registered_students: u16,
    pub seminars: Vec<Seminar>,
    pub teachers: Vec<Teacher>,
}

impl <T> Parseable for T where T: Creatable {
    fn parse(body: &str) -> Result<T, IsApiErrors> {
        DocumentParser::from_string(body)
            .map(|x| T::new(x.document()))
    }
}

impl Creatable for CourseInfo {
    fn new(doc: &dom::Document) -> CourseInfo {
        let doc = package.as_document();
        let base = String::from();
        let parser = utils::DocumentParser::from_document(&doc);
        let mut info = CourseInfo {
            faculty_id: parser.to_u32("/PREDMET_INFO/FAKULTA_ID"),
            faculty_shortcut: parser.to_string("/PREDMET_INFO/FAKULTA_ZKRATKA_DOM"),
            course_code: parser.to_string("/PREDMET_INFO/KOD_PREDMETU"),
            course_short_name: parser.to_string("/PREDMET_INFO/KRATKY_NAZEV_PREDMETU"),
            course_short_name_en: parser.to_string("/PREDMET_INFO/KRATKY_NAZEV_PREDMETU_ANGL"),
            course_name_en: parser.to_string("/PREDMET_INFO/NAZEV_PREDMETU_ANGL"),
            course_name: parser.to_string("/PREDMET_INFO/NAZEV_PREDMETU"),
            period_id: parser.to_u32("/PREDMET_INFO/OBDOBI_ID"),
            period_name: parser.to_string("/PREDMET_INFO/OBDOBI_NAZEV"),
            period_name_en: parser.to_string("/PREDMET_INFO/OBDOBI_NAZEV_ANGL"),
            period_url_shortcut: parser.to_string("/PREDMET_INFO/OBDOBI_ZKRATKA_PRO_URL"),
            period_url_shortcut_en: parser.to_string("/PREDMET_INFO/OBDOBI_ZKRATKA_PRO_URL_ANGL"),
            enrolled_students: parser.to_u16("/PREDMET_INFO/POCET_ZAREG_STUDENTU"),
            registered_students: parser.to_u16("/PREDMET_INFO/POCET_ZAREG_STUDENTU"),
            course_id: parser.to_u32("/PREDMET_INFO/PREDMET_ID"),
            seminars: vec![],
            teachers: vec![]
        };

        if let Value::Nodeset(nodes) = parser.value("/PREDMET_INFO/SEMINARE/SEMINAR") {
            for node in nodes.document_order() {
                if let Ok(seminar) = Seminar::parse(&node.string_value()) {
                    info.seminars.push(seminar)
                }
            }
        }

        if let Value::Nodeset(nodes) = parser.value("/PREDMET_INFO/VYUCUJICI_SEZNAM/VYUCUJICI") {
            for node in nodes.document_order() {
                if let Ok(teacher) = Teacher::parse(&node.string_value()) {
                    info.teachers.push(teacher)
                }
            }
        }

        info
    }
}



use crate::entities::Creatable;

#[derive(Debug)]
pub struct NotesClient {
    client: reqwest::Client,
    base_url: String,
    token: String,
    course: String,
    faculty: u16
}

impl NotesClient {
    pub fn new(url: &str, token: &str, faculty: u16, course: &str) -> NotesClient {
        NotesClient {
            client: reqwest::Client::new(),
            base_url: String::from(url),
            token: String::from(token),
            course: String::from(course),
            faculty
        }
    }

    pub fn course_info(&self) -> ResourceWrapper<CourseInfo> {

    }
}


#[derive(Debug)]
pub struct ResourceWrapper<T> where T: Creatable {
    response: reqwest::Response
}

impl <T> ResourceWrapper<T> {
    pub fn raw()
}


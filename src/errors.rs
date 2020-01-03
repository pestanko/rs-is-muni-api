use std::fmt;

pub trait IsApiError {
    fn message(&self) -> &str;
}

// Implement std::fmt::Display for IsApiError
impl <T> fmt::Display for T where T: IsApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, self.message()) // user-facing output
    }
}

// Implement std::fmt::Debug for IsApiError
impl <T> fmt::Debug for T where T: IsApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {}, message: {} }}", file!(), line!(), self.message())
    }
}

#[derive(From)]
pub enum IsApiErrors {
    HttpClientError(reqwest::Error),
    ParseError()
}

impl IsApiError for IsApiErrors {
    fn message(&self) -> &str {
        match *self {
            HttpClientError(err) => format!("Http Error: {}", err),
            ParseError(err) => format!("Parse Error: {}", err)
        }
    }
}

impl IsApiGeneralError {
    pub fn new(message: &str) -> IsApiGeneralError {
        IsApiGeneralError {message: String::from(message)}
    }
}

impl std::error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}



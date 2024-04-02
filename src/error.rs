use bat::PrettyPrinter;
use user_error::UFE;

// Some errors are similar across stucturs, such as failing to find an index, or converting into a
// CString or into a pointer. Standardizing the messages using these errors.
#[derive(Debug)]
pub enum GLUtilityError {
    FailedToConvertToCString(String),
    ErrorLog(String),
    CouldNotCreateErrorLog,
}

impl std::error::Error for GLUtilityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
impl std::fmt::Display for GLUtilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GLUtilityError::FailedToConvertToCString(source_string) => {
                let s = &source_string[0..15];
                write!(f, "Failed to convert the string: \"{}...\" to a CString", s)
            }
            GLUtilityError::ErrorLog(log) => {
                let log = PrettyPrinter::new()
                    .input_from_bytes(log.as_bytes())
                    .language("glsl")
                    .print()
                    .unwrap_or(log);
                write!(f, "{}", log)
            }
            GLUtilityError::CouldNotCreateErrorLog => {
                write!(
                    f,
                    "Error encountered, but could not create the error log :["
                )
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<GLUtilityError> for crate::GLError {
    fn from(error: GLUtilityError) -> Self {
        crate::GLError::Other(error)
    }
}

// Pretty Print - It's possible we error out before being a part of a GLProgram. This is because we
// need to initialize an OpenGL context, and pass a Window into the contructor of GLProgram
impl UFE for GLUtilityError {}

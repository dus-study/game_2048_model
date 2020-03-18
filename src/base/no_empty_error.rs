use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NoEmptyError;

impl fmt::Display for NoEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No empty spaces for new numbers")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for NoEmptyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

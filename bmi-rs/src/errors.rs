use crate::BmiResult;
use std::error::Error;
use std::fmt;

macro_rules! err {
    ($name:ident, $msg:literal) => {
        #[doc = $msg]
        #[doc = " error"]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $name;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $msg)
            }
        }

        impl Error for $name {}

        impl<T> From<$name> for BmiResult<T> {
            fn from(value: $name) -> Self {
                Err(Box::new(value))
            }
        }
    };
}

err!(BmiNotImplementedError, "not implemented");
err!(BmiIndexOutOfBounds, "index out of bounds");

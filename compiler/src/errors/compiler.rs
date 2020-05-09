use crate::errors::{FunctionError, ImportError, IntegerError};

use std::io;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("{}", _0)]
    ImportError(ImportError),

    #[error("{}", _0)]
    IntegerError(IntegerError),

    #[error("{}", _0)]
    FunctionError(FunctionError),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("Syntax error. Cannot parse the file")]
    FileParsingError,

    #[error("Main function not found")]
    NoMain,

    #[error("Main must be a function")]
    NoMainFunction,

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,

    #[error("writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        CompilerError::Crate("std::io", format!("{}", error))
    }
}

impl From<ImportError> for CompilerError {
    fn from(error: ImportError) -> Self {
        CompilerError::ImportError(error)
    }
}

impl From<IntegerError> for CompilerError {
    fn from(error: IntegerError) -> Self {
        CompilerError::IntegerError(error)
    }
}

impl From<FunctionError> for CompilerError {
    fn from(error: FunctionError) -> Self {
        CompilerError::FunctionError(error)
    }
}
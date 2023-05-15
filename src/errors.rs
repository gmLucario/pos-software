//! Create the custom app errors

use std::fmt::{self, Display};

use crate::result::AppResult;

#[derive(Debug, Default, Clone, Copy)]
pub enum ErrorType {
    #[default]
    SetUpError,
    DbError,
    ValidationError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ErrorType::SetUpError => "SetUpError",
            ErrorType::DbError => "DbError",
            ErrorType::ValidationError => "ValidationError",
        };
        write!(f, "{msg}")
    }
}

#[derive(Clone)]
pub struct AppError {
    pub component_name: String,
    pub msg: String,
    pub raw_msg: String,
    pub err_type: ErrorType,
}

impl AppError {
    fn get_error(component_name: &str, msg: &str, raw_msg: &str, err_type: ErrorType) -> Self {
        Self {
            component_name: component_name.to_string(),
            msg: msg.to_string(),
            raw_msg: raw_msg.to_string(),
            err_type,
        }
    }
    pub fn setup_error<T>(component_name: &str, msg: &str, raw_msg: &str) -> AppResult<T> {
        Err(Self::get_error(
            component_name,
            msg,
            raw_msg,
            ErrorType::SetUpError,
        ))
    }
    pub fn db_error<T>(component_name: &str, msg: &str, raw_msg: &str) -> AppResult<T> {
        Err(Self::get_error(
            component_name,
            msg,
            raw_msg,
            ErrorType::DbError,
        ))
    }
    pub fn validation_error<T>(component_name: &str, msg: &str, raw_msg: &str) -> AppResult<T> {
        Err(Self::get_error(
            component_name,
            msg,
            raw_msg,
            ErrorType::ValidationError,
        ))
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{error_type} {{ component_name: {component_name}, msg: {msg}, raw_msg: {raw_msg} }}",
            error_type = self.err_type,
            component_name = self.component_name,
            msg = self.msg,
            raw_msg = self.raw_msg
        )
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{error_type} {{ component_name: {component_name}, msg: {msg}, raw_msg: {raw_msg} }}",
            error_type = self.err_type,
            component_name = self.component_name,
            msg = self.msg,
            raw_msg = self.raw_msg
        )
    }
}

impl std::error::Error for AppError {}

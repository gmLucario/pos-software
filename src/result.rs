//! Generic app types

use crate::errors::AppError;

pub type GenericReturn<T> = Result<T, Box<dyn std::error::Error>>;
pub type AppResult<T> = Result<T, AppError>;

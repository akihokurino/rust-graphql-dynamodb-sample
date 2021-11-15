pub mod application;
pub mod cognite;
mod ddb;
pub mod domain;
pub mod ssm;

use aws_sdk_dynamodb::error::{DeleteItemError, GetItemError, PutItemError, ScanError};
use aws_sdk_dynamodb::SdkError;
use thiserror::Error as ThisErr;

#[derive(ThisErr, Debug, PartialOrd, PartialEq, Clone)]
pub enum AppError {
    #[error("不正なパラメーターです: {0}")]
    BadRequest(String),
    #[error("認証エラーです")]
    UnAuthenticate,
    #[error("禁止された行為です")]
    Forbidden,
    #[error("指定されたリソースが見つかりません")]
    NotFound,
    #[error("サーバーエラーです: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<String> for AppError {
    fn from(v: String) -> Self {
        Self::Internal(v)
    }
}

impl From<SdkError<PutItemError>> for AppError {
    fn from(e: SdkError<PutItemError>) -> Self {
        println!("{:?}", e);
        Self::Internal("DynamoDBの書き込みエラーです".to_string())
    }
}

impl From<SdkError<ScanError>> for AppError {
    fn from(e: SdkError<ScanError>) -> Self {
        println!("{:?}", e);
        Self::Internal("DynamoDBのスキャン読み込みエラーです".to_string())
    }
}

impl From<SdkError<GetItemError>> for AppError {
    fn from(e: SdkError<GetItemError>) -> Self {
        println!("{:?}", e);
        Self::Internal("DynamoDBのGet読み込みエラーです".to_string())
    }
}

impl From<SdkError<DeleteItemError>> for AppError {
    fn from(e: SdkError<DeleteItemError>) -> Self {
        println!("{:?}", e);
        Self::Internal("DynamoDBの削除エラーです".to_string())
    }
}

impl From<jsonwebtokens_cognito::Error> for AppError {
    fn from(_err: jsonwebtokens_cognito::Error) -> Self {
        Self::UnAuthenticate
    }
}

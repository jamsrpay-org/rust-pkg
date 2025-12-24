use sea_orm::{RuntimeErr, SqlxError};
use sea_orm_migration::DbErr;

#[derive(Debug, PartialEq, Eq)]
pub enum PgErrorCode {
    ForeignKeyViolation, // 23503
    UniqueViolation,     // 23505
    Other(String),       // Any other code
}

pub fn pg_error_from_db_err(err: &DbErr) -> Option<PgErrorCode> {
    match err {
        DbErr::Exec(RuntimeErr::SqlxError(SqlxError::Database(db_err)))
        | DbErr::Query(RuntimeErr::SqlxError(SqlxError::Database(db_err))) => {
            if let Some(code) = db_err.code() {
                match code.as_ref() {
                    "23503" => Some(PgErrorCode::ForeignKeyViolation),
                    "23505" => Some(PgErrorCode::UniqueViolation),
                    other => Some(PgErrorCode::Other(other.to_string())),
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn create_db_id() -> String {
    cuid2::create_id()
}

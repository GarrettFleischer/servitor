#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    #[error("unbalanced entry")]
    UnbalancedEntry,
    #[error("invalid entry")]
    InvalidEntry,
    #[error("not found")]
    NotFound,
    #[error("idempotency key reused with a different body")]
    IdempotencyKeyReuse,
    #[error("account name already exists for tenant")]
    DuplicateAccount,
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

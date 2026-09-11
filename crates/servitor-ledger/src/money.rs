use crate::error::LedgerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money(i64);

impl Money {
    pub fn from_cents(cents: i64) -> Result<Self, LedgerError> {
        if cents <= 0 {
            return Err(LedgerError::InvalidEntry);
        }
        Ok(Self(cents))
    }

    pub fn cents(self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Usd,
}

impl Currency {
    pub fn as_code(self) -> &'static str {
        match self {
            Self::Usd => "USD",
        }
    }

    pub fn parse(code: &str) -> Result<Self, LedgerError> {
        match code {
            "USD" => Ok(Self::Usd),
            _ => Err(LedgerError::InvalidEntry),
        }
    }
}

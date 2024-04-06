use anyhow::{ensure, Context, Result};
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub id: EntryId,
    pub body: EntryBody,
}

impl Entry {
    pub fn new(body: EntryBody) -> Self {
        Self {
            id: EntryId::new(),
            body: body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryId(Uuid);

impl EntryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<&str> for EntryId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let uuid = Uuid::parse_str(value).context("Failed to parse EntryId")?;
        Ok(Self(uuid))
    }
}

impl Into<String> for EntryId {
    fn into(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryBody(String);

impl EntryBody {
    pub fn new(body: String) -> Result<Self> {
        ensure!(!body.is_empty(), "Body must not be empty");
        ensure!(body.len() <= 1000, "Body must not exceed 1000 characters");

        Ok(Self(body))
    }
}

impl TryFrom<&str> for EntryBody {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value.to_string())
    }
}

impl Into<String> for EntryBody {
    fn into(self) -> String {
        self.0
    }
}

#[async_trait]
pub trait EntryRepository {
    async fn save(&self, entry: Entry) -> Result<()>;
    async fn get_by_ids(&self, id: &[EntryId]) -> Result<Vec<Entry>>;
}

pub trait ProvideEntryRepository {
    type Repository: EntryRepository;

    fn provide(&self) -> &Self::Repository;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_entry_body_new() {
        let body = EntryBody::new("Hello, World!".to_string());
        assert!(body.is_ok());

        let body = EntryBody::new("".to_string());
        assert!(body.is_err());

        let body = EntryBody::new("a".repeat(1001));
        assert!(body.is_err());
    }
}

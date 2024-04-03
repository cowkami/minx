use anyhow::{ensure, Context, Result};
use uuid:Uuid;

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
    type Error = uuid::Error;

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
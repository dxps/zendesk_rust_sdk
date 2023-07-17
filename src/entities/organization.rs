use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub details: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ListOrganizationsResp {
    pub organizations: Vec<Organization>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: i64,
    /// Is true if any comments are public, false otherwise.
    pub is_public: Option<bool>,
    pub brand_id: Option<i64>,
    pub group_id: Option<i64>,
    pub description: Option<String>,
    pub has_incidents: Option<bool>,
    pub organization_id: Option<i64>,
    /// The urgency with which the ticket should be addressed. Allowed values are "urgent", "high", "normal", or "low".
    pub priority: Option<String>,
    /// The user who requested this ticket.
    pub requester_id: i64,
    /// The state of the ticket. <br/>
    /// Allowed values are "new", "open", "pending", "hold", "solved", or "closed".<br/>
    /// If your account has activated custom ticket statuses, this is the ticket's status category.
    /// See [custom ticket statuses](https://developer.zendesk.com/api-reference/ticketing/tickets/tickets/#custom-ticket-statuses).
    pub status: Option<String>,
    pub subject: Option<String>,
    pub tags: Option<Vec<String>>,
    /// The type of this ticket. Allowed values are "problem", "incident", "question", or "task".
    pub r#type: Option<String>,
    pub url: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct GetTicketsResp {
    pub tickets: Vec<Ticket>,
}

#[derive(Debug, Deserialize)]
pub struct GetTicketsCountResp {
    pub count: GetTicketsCountRespBody,
}

#[derive(Debug, Deserialize)]
pub struct GetTicketsCountRespBody {
    pub value: i64,
    pub refreshed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SearchTicketsResp {
    pub results: Vec<Ticket>,
    pub next_page: Option<String>,
    pub previous_page: Option<String>,
    pub count: i32,
}

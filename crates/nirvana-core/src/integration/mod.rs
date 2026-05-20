pub(crate) mod jira;

use crate::api::domain::Connection;
use jira::JiraIntegration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntegrationError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("ticket not found: {0}")]
    TicketNotFound(String),
    #[error("authentication failed: {0}")]
    Auth(String),
    #[error("unsupported connection kind: {0}")]
    UnsupportedKind(String),
}

pub(crate) struct IssueInfo {
    pub summary: String,
}

enum IntegrationKind {
    Jira(JiraIntegration),
}

pub(crate) struct Integration {
    kind: IntegrationKind,
}

impl Integration {
    pub fn test_connection(&self) -> Result<(), IntegrationError> {
        match &self.kind {
            IntegrationKind::Jira(j) => j.test_connection(),
        }
    }

    pub fn fetch_issue(&self, ticket_key: &str) -> Result<IssueInfo, IntegrationError> {
        match &self.kind {
            IntegrationKind::Jira(j) => j.fetch_issue(ticket_key),
        }
    }

    pub fn publish_slot(
        &self,
        ticket_key: &str,
        started_at: i64,
        seconds: i64,
    ) -> Result<(), IntegrationError> {
        match &self.kind {
            IntegrationKind::Jira(j) => j.publish_slot(ticket_key, started_at, seconds),
        }
    }
}

pub(crate) fn build_integration(
    connection: &Connection,
    token: &str,
) -> Result<Integration, IntegrationError> {
    let kind = match connection.kind.as_str() {
        "jira-cloud" | "jira-dc" => IntegrationKind::Jira(JiraIntegration::new(
            &connection.host,
            &connection.identity,
            token,
        )),
        _ => return Err(IntegrationError::UnsupportedKind(connection.kind.clone())),
    };
    Ok(Integration { kind })
}

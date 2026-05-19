use crate::integration::{IntegrationError, IssueInfo};

pub(crate) struct JiraIntegration {
    host: String,
    identity: String,
    token: String,
    client: reqwest::blocking::Client,
}

impl JiraIntegration {
    pub(crate) fn new(host: &str, identity: &str, token: &str) -> Self {
        Self {
            host: host.to_string(),
            identity: identity.to_string(),
            token: token.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl JiraIntegration {
    pub(crate) fn fetch_issue(&self, ticket_key: &str) -> Result<IssueInfo, IntegrationError> {
        let url = format!(
            "https://{}/rest/api/3/issue/{}?fields=summary",
            self.host, ticket_key
        );

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.identity, Some(&self.token))
            .send()?;

        let status = response.status();
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(IntegrationError::TicketNotFound(ticket_key.to_string()));
        }
        if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
            return Err(IntegrationError::Auth(status.to_string()));
        }
        if !status.is_success() {
            return Err(IntegrationError::Network(
                response.error_for_status_ref().unwrap_err(),
            ));
        }

        let body: serde_json::Value = response.json()?;
        let summary = body["fields"]["summary"].as_str().unwrap_or("").to_string();

        Ok(IssueInfo { summary })
    }
}

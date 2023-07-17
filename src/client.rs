use std::io;

use base64::{engine::general_purpose, Engine};
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    group::ListGroupsResp,
    organization::ListOrganizationsResp,
    ticket::{GetTicketsCountResp, GetTicketsResp},
};

/// Zendesk v2 APIs [requests authentication options](https://support.zendesk.com/hc/en-us/articles/4408831452954-How-can-I-authenticate-API-requests-).
pub enum AuthCredential {
    ApiTokenCredential { email: String, api_token: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    base_url: String,
    email: String,
    api_token: String,
    oauth_token: Option<String>,
}

impl Client {
    /// Create a new `Client` for talking with Zendesk API.
    pub fn new(base_url: String, credential: AuthCredential) -> Self {
        match credential {
            AuthCredential::ApiTokenCredential { email, api_token } => {
                let oauth_token = format!("{}/token:{}", email, api_token);
                let oauth_token = general_purpose::STANDARD.encode(oauth_token);
                Self {
                    base_url,
                    email,
                    api_token,
                    oauth_token: Some(oauth_token),
                }
            }
        }
    }

    pub(crate) async fn do_request<T>(&self, method: Method, resource: String) -> Result<T, std::io::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", &self.base_url, resource);
        let request = reqwest::Client::new().request(method, url).header(
            "Authorization",
            format!("Basic {}", self.oauth_token.as_ref().unwrap()),
        );

        // Instead of doing `request.send().await?.json::<T>().await`, the response is explicitly
        // fetched and converted, so that it can be investigated in case of an error.
        match request.send().await {
            Ok(resp) => {
                let resp_body = resp.text().await.unwrap();
                log::trace!("[do_request] Got response {:?}", resp_body);
                match serde_json::from_str::<T>(&resp_body) {
                    Ok(resp_json) => Ok(resp_json),
                    Err(err) => {
                        log::debug!("Got a deserialization error for this string: {resp_body}");
                        Err(io::Error::new(io::ErrorKind::Other, err))
                    }
                }
            }
            Err(err) => {
                log::error!("[do_request] Failed with '{err}'.");
                Err(io::Error::new(io::ErrorKind::Other, err))
            }
        }
    }

    pub async fn list_groups(&self) -> Result<ListGroupsResp, io::Error> {
        self.do_request::<ListGroupsResp>(reqwest::Method::GET, format!("groups"))
            .await
    }

    pub async fn list_organizations(&self) -> Result<ListOrganizationsResp, io::Error> {
        self.do_request::<ListOrganizationsResp>(reqwest::Method::GET, format!("organizations"))
            .await
    }

    pub async fn get_tickets(&self) -> Result<GetTicketsResp, io::Error> {
        self.do_request::<GetTicketsResp>(reqwest::Method::GET, format!("tickets"))
            .await
    }

    pub async fn get_tickets_count(&self) -> Result<GetTicketsCountResp, io::Error> {
        self.do_request::<GetTicketsCountResp>(reqwest::Method::GET, format!("tickets/count"))
            .await
    }
}

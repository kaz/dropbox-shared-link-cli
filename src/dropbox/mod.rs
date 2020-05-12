mod token;
mod types;

pub use types::SharedToken;
use types::*;

pub struct SharedLinkClient {
    client: reqwest::Client,
    token: String,
}

impl SharedLinkClient {
    pub fn new() -> SharedLinkClient {
        SharedLinkClient {
            client: reqwest::Client::new(),
            token: token::generate(),
        }
    }

    pub async fn list(
        &self,
        shared_token: &SharedToken,
    ) -> Result<ListResult, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .post("https://www.dropbox.com/list_shared_link_folder_entries")
            .form(&[
                ("t", &self.token),
                ("link_type", &shared_token.link_type),
                ("link_key", &shared_token.link_key),
                ("secure_hash", &shared_token.secure_hash),
                ("sub_path", &shared_token.sub_path),
            ])
            .header("Cookie", ["t", &self.token].join("="))
            .send()
            .await?;

        resp.error_for_status_ref()?;
        Ok(resp.json().await?)
    }
}

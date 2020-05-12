mod token;
mod types;

pub use types::ShareToken;
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

    pub async fn list(&self, share: &ShareToken) -> Result<ListResult, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .post("https://www.dropbox.com/list_shared_link_folder_entries")
            .form(&[
                ("t", &self.token),
                ("link_type", &share.link_type),
                ("link_key", &share.link_key),
                ("secure_hash", &share.secure_hash),
                ("sub_path", &share.sub_path),
            ])
            .header("Cookie", ["t", &self.token].join("="))
            .send()
            .await?;

        resp.error_for_status_ref()?;
        Ok(resp.json().await?)
    }
}

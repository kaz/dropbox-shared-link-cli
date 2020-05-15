#[macro_use]
mod error;
mod token;
mod types;

use types::*;

pub struct SharedLinkClient {
    client: reqwest::Client,
    token: String,
    root: ShareToken,
}

impl SharedLinkClient {
    pub fn new<S>(url: S) -> Result<SharedLinkClient, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        Ok(SharedLinkClient {
            client: reqwest::Client::new(),
            token: token::generate(),
            root: ShareToken::from_url(url).ok_or("failed to parse specified URL")?,
        })
    }

    async fn list(&self, share: &ShareToken) -> Result<ListResult, Box<dyn std::error::Error>> {
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

    #[async_recursion::async_recursion]
    async fn get_entry(
        &self,
        base: &ShareToken,
        path: &std::path::Path,
    ) -> Result<(Entry, ShareToken), Box<dyn std::error::Error>> {
        let result = self.list(base).await?;

        // to find root folder
        if path.eq(std::path::Path::new(
            match result.folder_share_token.sub_path.as_ref() {
                "" => "/",
                s => s,
            },
        )) {
            return Ok((result.folder, result.folder_share_token));
        }

        for (ent, st) in result.entries.into_iter().zip(result.share_tokens) {
            let current = std::path::Path::new(&st.sub_path);
            if path.eq(current) {
                return Ok((ent, st));
            }
            if path.starts_with(current) {
                return self.get_entry(&st, path).await;
            }
        }

        Err(error::emit("not found"))
    }

    pub async fn ls<S>(&self, path: S) -> Result<Vec<Entry>, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        Ok(self
            .list(
                &self
                    .get_entry(&self.root, std::path::Path::new(&path.into()))
                    .await?
                    .1,
            )
            .await?
            .entries)
    }
}

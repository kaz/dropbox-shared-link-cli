#[macro_use]
mod error;
mod token;
mod types;

use types::*;

pub struct SharedLinkClient {
    client: reqwest::blocking::Client,
    token: String,
    root: ShareToken,
}

impl SharedLinkClient {
    pub fn new<S>(url: S) -> Result<SharedLinkClient, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        Ok(SharedLinkClient {
            client: reqwest::blocking::Client::new(),
            token: token::generate(),
            root: ShareToken::from_url(url).ok_or("failed to parse specified URL")?,
        })
    }

    fn list(&self, share: &ShareToken) -> Result<ListResult, Box<dyn std::error::Error>> {
        let mut results: Vec<ListResult> = vec![];
        let mut voucher = String::new();

        loop {
            let mut params = vec![
                ("t", &self.token),
                ("link_type", &share.link_type),
                ("link_key", &share.link_key),
                ("secure_hash", &share.secure_hash),
                ("sub_path", &share.sub_path),
            ];
            if voucher != "" {
                params.push(("voucher", &voucher));
            }

            let resp = self
                .client
                .post("https://www.dropbox.com/list_shared_link_folder_entries")
                .header("Cookie", ["t", &self.token].join("="))
                .form(&params)
                .send()?;

            resp.error_for_status_ref()?;

            let api_result = resp.json::<ListAPIResult>()?;
            results.push(api_result.data);

            voucher = match api_result.next_request_voucher {
                Some(v) => v,
                None => break,
            };
        }

        let mut result = results.pop().ok_or("no results")?;
        while results.len() > 0 {
            let r = results.pop().unwrap();
            result.entries.extend(r.entries);
            result.share_tokens.extend(r.share_tokens);
        }

        Ok(result)
    }

    fn get_entry(
        &self,
        base: &ShareToken,
        path: &std::path::Path,
    ) -> Result<(Entry, ShareToken), Box<dyn std::error::Error>> {
        let result = self.list(base)?;

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
                return self.get_entry(&st, path);
            }
        }

        Err(error::emit("not found"))
    }

    pub fn ls<S>(&self, path: S) -> Result<Vec<Entry>, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        Ok(self
            .list(
                &self
                    .get_entry(&self.root, std::path::Path::new(&path.into()))?
                    .1,
            )?
            .entries)
    }
}

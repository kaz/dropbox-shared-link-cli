#[macro_use]
mod error;
mod token;
mod types;

use types::*;

struct List<'a> {
    client: &'a SharedLinkClient,
    voucher: Option<String>,
    pwd: SharedEntity,
    iter: Box<dyn std::iter::Iterator<Item = SharedEntity>>,
}

impl<'a> List<'a> {
    fn fetch(&mut self) -> Option<SharedEntity> {
        let resp = self.client.list_iter(
            &self.pwd.1,
            match self.voucher.clone() {
                None => return None,
                v => v,
            },
        );

        self.voucher = resp.voucher;
        self.pwd = resp.pwd;

        self.iter = resp.iter;
        self.iter.next()
    }
}

impl<'a> std::iter::Iterator for List<'a> {
    type Item = SharedEntity;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => self.fetch(),
            e => e,
        }
    }
}

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

    fn call_list_api(
        &self,
        share: &ShareToken,
        voucher: Option<String>,
    ) -> Result<ListAPIResult, Box<dyn std::error::Error>> {
        let mut params = vec![
            ("t", &self.token),
            ("link_type", &share.link_type),
            ("link_key", &share.link_key),
            ("secure_hash", &share.secure_hash),
            ("sub_path", &share.sub_path),
        ];
        if let Some(voucher) = &voucher {
            params.push(("voucher", voucher));
        }

        let resp = self
            .client
            .post("https://www.dropbox.com/list_shared_link_folder_entries")
            .header("Cookie", ["t", &self.token].join("="))
            .form(&params)
            .send()?;

        resp.error_for_status_ref()?;

        Ok(resp.json::<ListAPIResult>()?)
    }

    fn list_iter(&self, share: &ShareToken, voucher: Option<String>) -> List {
        match self.call_list_api(share, voucher) {
            Ok(s) => List {
                client: self,
                voucher: (&s).next_request_voucher.clone(),
                pwd: s.pwd(),
                iter: Box::new(s.entities()),
            },
            Err(e) => todo!("TODO"),
        }
    }

    fn get_entry(
        &self,
        base: &ShareToken,
        path: &std::path::Path,
    ) -> Result<SharedEntity, Box<dyn std::error::Error>> {
        let result = self.list_iter(base, None);

        // to find root folder
        if path.eq(std::path::Path::new(match result.pwd.1.sub_path.as_ref() {
            "" => "/",
            s => s,
        })) {
            return Ok(result.pwd);
        }

        for (ent, st) in result {
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
            .list_iter(
                &self
                    .get_entry(&self.root, std::path::Path::new(&path.into()))?
                    .1,
                None,
            )
            .map(|x| x.0)
            .collect())
    }
}

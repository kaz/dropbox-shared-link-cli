#[macro_use]
mod error;
mod token;
mod types;

use types::*;

type SharedEntryResult = Result<SharedEntity, Box<dyn std::error::Error>>;

struct SharedEntries<'a> {
    client: &'a SharedLinkClient,
    voucher: Option<String>,
    pwd: Option<SharedEntity>,
    iter: Box<dyn std::iter::Iterator<Item = SharedEntryResult>>,
}

impl<'a> SharedEntries<'a> {
    fn fetch(&mut self) -> Option<SharedEntryResult> {
        let resp = self.client.entities(
            match &self.pwd {
                None => return None,
                Some(v) => &v.1,
            },
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

impl<'a> std::iter::Iterator for SharedEntries<'a> {
    type Item = SharedEntryResult;
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

    fn entities(&self, share: &ShareToken, voucher: Option<String>) -> SharedEntries {
        match self.call_list_api(share, voucher) {
            Ok(s) => SharedEntries {
                client: self,
                voucher: (&s).next_request_voucher.clone(),
                pwd: Some(s.pwd()),
                iter: Box::new(s.entities().map(|x| Ok(x))),
            },
            Err(e) => SharedEntries {
                client: self,
                voucher: None,
                pwd: None,
                iter: Box::new(vec![Err(e)].into_iter()),
            },
        }
    }

    fn entry(
        &self,
        base: &ShareToken,
        path: &std::path::Path,
    ) -> Result<SharedEntity, Box<dyn std::error::Error>> {
        let entites = self.entities(base, None);

        // to find root folder
        if let Some(pwd) = &entites.pwd {
            if path.eq(std::path::Path::new(match pwd.1.sub_path.as_ref() {
                "" => "/",
                s => s,
            })) {
                return Ok(pwd.clone());
            }
        }

        for x in entites {
            let (ent, st) = x?;
            let current = std::path::Path::new(&st.sub_path);
            if path.eq(current) {
                return Ok((ent, st));
            }
            if path.starts_with(current) {
                return self.entry(&st, path);
            }
        }

        Err(error::emit(format!(
            "`{}` was not found",
            path.to_string_lossy()
        )))
    }

    fn find(&self, path: String) -> Result<SharedEntity, Box<dyn std::error::Error>> {
        self.entry(&self.root, std::path::Path::new(&path))
    }

    pub fn ls<S>(&self, path: S) -> Result<Vec<Entry>, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        Ok(self
            .entities(&self.find(path.into())?.1, None)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|x| x.0)
            .collect())
    }

    pub fn get<S>(&self, path: S) -> Result<(), Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let path = path.into();
        let ent = self.find(path.clone())?.0;
        if ent.is_dir {
            return Err(error::emit(format!("`{}` is directory", path)));
        }

        println!("{}", ent.href);
        Ok(())
    }
}

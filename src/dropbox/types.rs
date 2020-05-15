pub type SharedEntity = (Entry, ShareToken);

#[derive(serde::Deserialize, Debug)]
pub struct ListAPIResult {
    pub next_request_voucher: Option<String>,

    folder: Entry,
    pub folder_share_token: ShareToken,

    entries: Vec<Entry>,
    share_tokens: Vec<ShareToken>,
}

impl ListAPIResult {
    pub fn pwd(&self) -> SharedEntity {
        (self.folder.clone(), self.folder_share_token.clone())
    }
    pub fn entities(self) -> impl std::iter::Iterator<Item = SharedEntity> {
        self.entries.into_iter().zip(self.share_tokens)
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Entry {
    pub is_dir: bool,
    pub href: String,
    pub filename: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShareToken {
    pub link_type: String,
    pub link_key: String,
    pub secure_hash: String,
    pub sub_path: String,
}

impl ShareToken {
    fn new<S>(link_type: S, link_key: S, secure_hash: S, sub_path: S) -> ShareToken
    where
        S: Into<String>,
    {
        ShareToken {
            link_type: link_type.into(),
            link_key: link_key.into(),
            secure_hash: secure_hash.into(),
            sub_path: sub_path.into(),
        }
    }

    pub fn from_url<S>(url: S) -> Option<ShareToken>
    where
        S: Into<String>,
    {
        let s = url.into();
        let v = s.split("/").collect::<Vec<_>>();

        if v.len() < 6
            || v[0] != "https:"
            || v[1] != ""
            || v[2] != "www.dropbox.com"
            || v[3] != "sh"
        {
            return None;
        }

        Some(ShareToken::new(
            "s",
            v[4],
            v[5],
            &format!("/{}", v[6..].join("/")),
        ))
    }
}

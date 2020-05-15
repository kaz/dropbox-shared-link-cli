#[derive(serde::Deserialize, Debug)]
pub struct ListResult {
    pub folder: Entry,
    pub folder_share_token: ShareToken,
    pub entries: Vec<Entry>,
    pub share_tokens: Vec<ShareToken>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Entry {
    pub is_dir: bool,
    pub href: String,
    pub filename: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShareToken {
    pub link_type: String,
    pub link_key: String,
    pub secure_hash: String,
    pub sub_path: String,
}

impl ShareToken {
    pub fn new<S>(link_type: S, link_key: S, secure_hash: S, sub_path: S) -> ShareToken
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

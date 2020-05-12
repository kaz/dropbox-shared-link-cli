#[derive(serde::Deserialize, Debug)]
pub struct ListResult {
    share_tokens: Vec<SharedToken>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedToken {
    pub link_type: String,
    pub link_key: String,
    pub sub_path: String,
    pub secure_hash: String,
}

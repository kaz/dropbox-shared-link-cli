mod dropbox;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = dropbox::SharedToken {
        link_type: "s".into(),
        link_key: "arnpe0ef5wds8cv".into(),
        secure_hash: "AAAk_SECQ2Nc6SVGii3rHX6Fa".into(),
        sub_path: "/".into(),
    };

    let client = dropbox::SharedLinkClient::new();
    let resp = client.list(&base_dir).await?;

    println!("{:#?}", resp);
    Ok(())
}

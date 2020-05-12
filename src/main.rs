mod dropbox;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = dropbox::ShareToken::from_url(
        "https://www.dropbox.com/sh/arnpe0ef5wds8cv/AAAk_SECQ2Nc6SVGii3rHX6Fa/",
    )
    .expect("failed to parse URL");

    let client = dropbox::SharedLinkClient::new();
    let resp = client.list(&base_dir).await?;

    println!("{:#?}", resp);
    Ok(())
}

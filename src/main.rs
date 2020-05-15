mod dropbox;

fn main() {
    let client = dropbox::SharedLinkClient::new(
        "https://www.dropbox.com/sh/arnpe0ef5wds8cv/AAAk_SECQ2Nc6SVGii3rHX6Fa/",
    )
    .expect("failed to setup client");

    let lss = client.ls("/").expect("failed to run ls");
    println!("{:?}", lss);
}

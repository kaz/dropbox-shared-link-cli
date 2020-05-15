mod dropbox;

use clap::Clap;

#[derive(Clap, Debug)]
struct Opts {
    /// Root URL
    #[clap(
        short,
        long,
        default_value = "https://www.dropbox.com/sh/arnpe0ef5wds8cv/AAAk_SECQ2Nc6SVGii3rHX6Fa/"
    )]
    root: String,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(clap::Clap, Debug)]
enum SubCommand {
    Ls(Ls),
    Get(Get),
}

/// list files/directories
#[derive(Clap, Debug)]
struct Ls {
    /// Where to ls (e.g. /ABC123/D )
    path: String,
}

/// download file
#[derive(Clap, Debug)]
struct Get {
    /// Which file to download (e.g. /ABC123/D/in/in01.txt )
    path: String,
}

fn main() {
    let opts = Opts::parse();

    let client = dropbox::SharedLinkClient::new(opts.root)
        .expect("failed to setup client: invalid root URL");

    match opts.subcmd {
        SubCommand::Ls(Ls { path }) => {
            for ent in client.ls(path).expect("failed to ls") {
                println!("{} {}", if ent.is_dir { "D" } else { "F" }, ent.filename)
            }
        }
        SubCommand::Get(Get { path }) => client.get(path).expect("failed to get"),
    }
}

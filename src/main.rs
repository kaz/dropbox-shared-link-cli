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

#[derive(Clap, Debug)]
enum SubCommand {
    Ls(Ls),
    Cp(Cp),
}

/// list files/directories
#[derive(Clap, Debug)]
struct Ls {
    /// Which directory to list (e.g. /ABC123/D )
    path: String,
}

/// download file
#[derive(Clap, Debug)]
struct Cp {
    /// Which file to download (e.g. /ABC123/D/in/in01.txt )
    remote_path: String,

    /// Where to save file (e.g. $HOME/Downloads/in.txt )
    local_path: String,
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
        SubCommand::Cp(Cp {
            remote_path,
            local_path,
        }) => {
            println!(
                "{} bytes downloaded",
                client.cp(remote_path, local_path).expect("failed to get")
            );
        }
    }
}

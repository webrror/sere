use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{
    net_protocol::Blobs,
    rpc::client::blobs::{ReadAtLen, WrapOption},
    ticket::BlobTicket,
    util::SetTagOption,
};

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    let blobs = Blobs::memory().build(&endpoint);

    let node = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .spawn()
        .await?;

    let client = blobs.client();

    let args = std::env::args().collect::<Vec<_>>();

    match &args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [_cmd, "send", path] => {
            println!();
            println!("\x1b[1m#### SEND FILE ####\x1b[0m");
            println!();
            let abs_path = PathBuf::from_str(path)?.canonicalize()?;

            if abs_path.is_file() {
                println!("File exists at \x1b[1m{}\x1b[0m", abs_path.display());

                println!();

                println!("Analyzing file...");

                let blob = client
                    .add_from_path(abs_path, true, SetTagOption::Auto, WrapOption::NoWrap)
                    .await?
                    .finish()
                    .await?;

                let node_id = node.endpoint().node_id();
                let ticket = BlobTicket::new(node_id.into(), blob.hash, blob.format)?;

                println!("File analyzed.");
                println!();
                println!("Fetch this file by running following command:");
                println!();
                println!("\x1b[1mcargo run receive {ticket} {path}\x1b[0m");
                println!();
                println!("Press Ctrl+C to exit.");
                tokio::signal::ctrl_c().await?;
            }
        }
        [_cmd, "receive", ticket, path] => {
            let path_buf: PathBuf = PathBuf::from_str(path)?;
            let ticket = BlobTicket::from_str(ticket)?;

            println!();
            println!("\x1b[1m#### RECEIVE FILE ####\x1b[0m");
            println!();

            println!("Downloading file...");
            client
                .download(ticket.hash(), ticket.node_addr().clone())
                .await?
                .finish()
                .await?;

            println!("File downloaded.");
            println!();

            println!("Writing file to \x1b[1m{}\x1b[0m...", path_buf.display());
            let mut file = tokio::fs::File::create(path_buf).await?;
            let mut reader = client.read_at(ticket.hash(), 0, ReadAtLen::All).await?;
            tokio::io::copy(&mut reader, &mut file).await?;

            println!("Finished writing file.");
        }
        _ => {
            println!("Could not parse args.");
            println!("Usage:");
            println!("  # to send:");
            println!("  cargo run --example transfer --send [FILE]");
            println!("  # to receive:");
            println!("  cargo run --example transfer --receive [HASH] [FILE]");
        }
    }

    println!("Shutting down sere");
    node.shutdown().await?;
    Ok(())
}

use clap::ArgGroup;
use std::path::PathBuf;
use clap::{Parser};
use fritzbox_presence::{auth, get_network_devices};


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Username for login
    #[arg()]
    username: String,
    /// Password for login
    #[arg()]
    password: String,
    /// The base url of the fritzbox
    #[arg()]
    base_url: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    let base_url = cli.base_url.unwrap_or_else(|| "http://fritz.box".to_string());

    let sid = auth(
        &base_url,
        &cli.username,
        &cli.password
    ).await;
    
    match sid {
        Ok(s) => {
            match get_network_devices(&base_url, &s).await {
                Ok(d) => {
                    for device in d {
                        println!("{}: {}", device.nameinfo.name, device.ipinfo)
                    }
                }
                Err(e) => {eprintln!("{:?}", e)}
            }
        },
        Err(e) => {eprintln!("{:?}", e)}
    }

}

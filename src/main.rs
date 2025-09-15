use fritzbox_presence::{auth, get_network_devices, AuthError, DataError};

#[tokio::main]
async fn main() {
    let sid = auth(
        "<url>",
        "user",
        "pw"
    ).await;

    match sid {
        Ok(s) => {
            match get_network_devices("<url>", &s).await {
                Ok(d) => {
                    todo!()
                    // for device in d {
                    //     println!("{}: {}", device.nameinfo.name, device.ipinfo)
                    // }
                }
                Err(e) => {eprintln!("{:?}", e)}
            }
        },
        Err(e) => {eprintln!("{:?}", e)}
    }

}

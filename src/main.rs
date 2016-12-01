extern crate allenap_libtftp;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use allenap_libtftp as tftp;
mod offload;


fn main() {
    let address = IpAddr::from_str("::").unwrap();
    let handler = offload::OffloadHandler{
        sockpath: "offload.sock".to_owned(),
    };
    match tftp::serve(SocketAddr::new(address, 69), &handler) {
        Ok(_) => println!("All okay."),
        Err(error) => println!("Broke: {}", error),
    };
}

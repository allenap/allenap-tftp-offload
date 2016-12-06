extern crate allenap_libtftp;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_term;

use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::process;
use std::str::FromStr;

use allenap_libtftp as tftp;
use clap::{Arg, App};
use slog::DrainExt;

mod offload;


fn main() {
    let matches = App::new("allenap's TFTP offloader")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Takes the load off serving TFTP, typically for MAAS.")
        .arg(Arg::with_name("socket")
             .short("s")
             .long("socket")
             .value_name("FILE")
             .help("The offload socket to use")
             .takes_value(true))
        .arg(Arg::with_name("address")
             .short("a")
             .long("address")
             .value_name("ADDRESS")
             .help("The address on which to listen")
             .takes_value(true))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .value_name("PORT")
             .help("The port on which to listen")
             .takes_value(true))
        .get_matches();

    let socket = matches.value_of("socket").unwrap_or("offload.sock");
    let address = matches.value_of("address").unwrap_or("::");
    let port = matches.value_of("port").unwrap_or("69");

    let address = match IpAddr::from_str(address) {
        Ok(address) => address,
        Err(error) => {
            writeln!(
                std::io::stderr(), "Could not parse {} as an address: {}",
                address, error).ok();
            process::exit(1);
        },
    };

    let port: u16 = match port.parse() {
        Ok(port) => port, Err(error) => {
            writeln!(
                std::io::stderr(), "Could not parse {} as a port: {}",
                port, error).ok();
            process::exit(1);
        },
    };


    let logger = slog::Logger::root(
        slog_term::streamer().build().fuse(), None);

    let sockaddr = SocketAddr::new(address, port);
    let handler = offload::OffloadHandler{
        sockpath: socket.to_owned(),
        logger: logger.clone(),
    };

    match tftp::serve(sockaddr, &handler, &logger) {
        Ok(_) => {
            writeln!(std::io::stdout(), "All okay.").ok();
            process::exit(0);
        },
        Err(error) => {
            writeln!(std::io::stderr(), "Something broke: {}", error).ok();
            process::exit(2);
        },
    };
}

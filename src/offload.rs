extern crate allenap_libtftp;
extern crate slog;

use std::fs;
use std::io::{Read, Write};
use std::io;
use std::net;
use std::os::unix::net::UnixStream;
use std::thread;

use allenap_libtftp as tftp;
use allenap_libtftp::options::Options;
use allenap_libtftp::packet::{Filename, Packet, TransferMode};
use allenap_libtftp::packet;
use allenap_libtftp::rrq;
use slog::Logger;


pub struct OffloadHandler {
    pub sockpath: String,
    pub logger: Logger,
}


impl tftp::Handler for OffloadHandler {

    fn handle_rrq(
        &self, local: net::SocketAddr, remote: net::SocketAddr,
        filename: Filename, txmode: TransferMode, options: Options)
        -> Option<Packet>
    {
        match query_offload(&self.sockpath, local, remote, &filename.0) {
            Ok(offload_file) => {
                let logger = self.logger.clone();
                thread::spawn(move|| {
                    let filename = Filename(offload_file.filename.clone());
                    rrq::serve_file(
                        remote, filename, txmode, options, &logger);
                    if offload_file.ephemeral {
                        fs::remove_file(offload_file.filename).ok();
                    }
                });
                None
            },
            Err(error) => {
                println!("{}", error);
                Some(Packet::Error(
                    packet::ErrorCode::NotDefined,
                    packet::ErrorMessage(
                        "could not query offload backend".to_owned()),
                ))
            },
        }
    }

}


#[derive(Debug)]
struct OffloadFile {
    filename: String,
    ephemeral: bool,
}


fn query_offload(
    sockpath: &str, local: net::SocketAddr, remote: net::SocketAddr,
    filename: &str)
    -> io::Result<OffloadFile>
{
    let mut stream = UnixStream::connect(sockpath)?;
    let request = format!("{}\0{}\0{}\0$", local.ip(), remote.ip(), filename);
    stream.write_all(&request.into_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    let mut parts = response.split("\0");
    let status = parts.next().ok_or_else(|| io::Error::new(
        io::ErrorKind::UnexpectedEof, "missing status"))?;
    if status == "-" {
        let eph = parts.next().ok_or_else(|| io::Error::new(
            io::ErrorKind::UnexpectedEof, "missing ephemeral flag"))?;
        let filename = parts.next().ok_or_else(|| io::Error::new(
            io::ErrorKind::UnexpectedEof, "missing filename"))?;
        let over = parts.next().ok_or_else(|| io::Error::new(
            io::ErrorKind::UnexpectedEof, "missing terminator"))?;
        if over != "$" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, format!(
                    "mangled terminator: {:?}", over)));
        }
        return Ok(OffloadFile{
            filename: filename.to_owned(),
            ephemeral: eph == "EPH",
        });
    }
    else {
        // TODO: Parse status as TFTP error code, and return it.
        let message = parts.next().ok_or_else(|| io::Error::new(
            io::ErrorKind::UnexpectedEof, "missing error message"))?;
        let error = io::Error::new(io::ErrorKind::Other, message);
        // TODO: Check terminator ("over"); see above.
        return Err(error);
    }
}

use std::net::UdpSocket;
use dns_server::Result;
use dns_server::helpers;

fn main() -> Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0",2053))?;
    loop {
        match helpers::handle_query(&socket){
            Ok(_) => {},
            Err(e) => eprintln!("An error occured: {}",e),
        }
    }
}
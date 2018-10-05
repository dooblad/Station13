extern crate game;

pub mod net;

use game::net::*;

use net::Server;

fn main() {
    use std::{thread, time};

    let mut server = Server::new(to_socket_addr(BIND_ADDR, SERVER_PORT));

    let ten_millis = time::Duration::from_millis(10);
    loop {
        server.tick();
        thread::sleep(ten_millis);
    }
}

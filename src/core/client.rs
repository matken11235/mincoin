use std::net::{SocketAddr, IpAddr, Ipv4Addr, ToSocketAddrs};

use p2p::connection_manager_4edge::ConnectionManager4Edge;

use super::server::{State, get_my_ip};

pub struct Client {
    server_state: State,
    cm: ConnectionManager4Edge,
}

impl Client {
    pub fn new(my_port: u16, core_addr: &'static str) -> Client {
        println!("Initializing ClientCore ...");
        const MY_IP: Ipv4Addr = get_my_ip();
        println!("Server IP address is set to ... {}", MY_IP);
        let my_addr = SocketAddr::new(IpAddr::V4(MY_IP), my_port);

        let core_addr = core_addr.to_socket_addrs().unwrap().next().unwrap();

        Client {
            server_state: State::Init,
            cm: ConnectionManager4Edge::new(my_addr, core_addr),
        }
    }

    pub fn start(&mut self) {
        self.server_state = State::Active;
        self.cm.start();
        self.cm.connect_to_core_node();
    }

    #[allow(dead_code)]
    pub fn get_my_current_state(&self) -> State {
        self.server_state.clone()
    }
}

impl Drop for Client {
    fn drop(&mut self) -> () { // shutdown_server
        self.server_state = State::ShuttingDown;
        println!("Shutdown edge node ...");
    }
}

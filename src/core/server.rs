use std::net::{SocketAddr, IpAddr, Ipv4Addr, ToSocketAddrs};
use p2p::connection_manager::ConnectionManager;

#[derive(Clone)]
pub enum State {
    Init,
    Standby,
    Active,
    ConnectedToNetwork,
    ShuttingDown,
}

pub struct Server {
    server_state: State,
    cm: ConnectionManager,
    core_node_addr: Option<SocketAddr>,
}

pub trait Overload<T> {
    fn new(_: T) -> Server;
}

impl Overload<u16> for Server {
    fn new(my_port: u16) -> Server {
        println!("Initializing server ...");
        const MY_IP: Ipv4Addr = get_my_ip();
        println!("Server IP address is set to ... {}", MY_IP);
        let my_addr = SocketAddr::new(IpAddr::V4(MY_IP), my_port);

        Server {
            server_state: State::Init,
            cm: ConnectionManager::new(my_addr),
            core_node_addr: None,
        }
    }
}

impl Overload<(u16, &'static str)> for Server {
    fn new(args: (u16, &'static str)) -> Server {
        let my_port = args.0;
        let node_addr = args.1.to_socket_addrs().unwrap().next().unwrap();

        println!("Initializing server ...");
        const MY_IP: Ipv4Addr = get_my_ip();
        println!("Server IP address is set to ... {}", MY_IP);
        let my_addr = SocketAddr::new(IpAddr::V4(MY_IP), my_port);

        Server {
            server_state: State::Init,
            cm: ConnectionManager::new(my_addr),
            core_node_addr: Some(node_addr),
        }
    }
}

impl Server {
    pub fn start(&mut self) {
        self.server_state = State::Standby;
        self.cm.start();
    }

    pub fn join_network(&mut self) {
        match self.core_node_addr {
            Some(addr) => {
                self.server_state = State::ConnectedToNetwork;
                self.cm.join_network(addr);
            },
            None => println!("This server is runnning as Genesis Core Node ..."),
        };
    }

    #[allow(dead_code)]
    pub fn get_my_current_state(&self) -> State {
        self.server_state.clone()
    }
}

impl Drop for Server {
    fn drop(&mut self) -> () { // shutdown_server
        self.server_state = State::ShuttingDown;
        println!("Shutdown server ...");
    }
}

pub const fn get_my_ip() -> Ipv4Addr {
    Ipv4Addr::LOCALHOST
}
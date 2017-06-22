extern crate varlink;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod io_systemd_network {
    /*
    varlink_server! (r#"
    # Provides information about network state
	interface io.systemd.network

	type NetdevInfo (
	  ifindex: int,
	  ifname: string
	)

	type Netdev (
	  ifindex: int,
	  ifname: string
	)

	# Returns information about a network device
	method Info(ifindex: int) -> (info: NetdevInfo)

	# Lists all network devices
	method List() -> (netdevs: Netdev[])

	error UnknownNetworkDevice
	error InvalidParameter
    "#);
    */

    // will be autogenerated by the macro in the future
    include!("io_systemd_network/mod.rs");

    use std::sync::{Arc, RwLock};
    pub struct Server {
        pub state: Arc<RwLock<i64>>,
    }
}

extern crate tokio_proto;

use varlink::server::{Proto, VarlinkService};
use tokio_proto::TcpServer;

use io_systemd_network::{Error, InfoRet, Netdev, NetdevInfo, ListRet, Interface, Server};
use std::sync::{Arc, RwLock};

// bring the Interface trait in scope
use varlink::server::Interface as _I;

impl Interface for Server {
    fn info(&self, i: i64) -> Result<InfoRet, Error> {
        // State example
        {
            let mut number = self.state.write().unwrap();

            *number += 1;

            println!("{}", *number);
        }

        match i {
            1 => {
                Ok(InfoRet {
                       info: NetdevInfo {
                           ifindex: 1,
                           ifname: "lo".into(),
                       },
                   })
            }
            2 => {
                Ok(InfoRet {
                       info: NetdevInfo {
                           ifindex: 2,
                           ifname: "eth0".into(),
                       },
                   })
            }
            _ => Err(Error::UnknownNetworkDevice),
        }
    }

    fn list(&self) -> Result<ListRet, Error> {
        // State example
        {
            let mut number = self.state.write().unwrap();

            *number -= 1;

            println!("{}", *number);
        }
        Ok(ListRet {
               netdevs: Some(vec![Netdev {
                                      ifindex: 1,
                                      ifname: "lo".into(),
                                  },
                                  Netdev {
                                      ifindex: 2,
                                      ifname: "eth0".into(),
                                  }]),
           })
    }
}

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(Proto, addr);

    let state = Arc::new(RwLock::new(0));

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(move || {
                     println!("New Server");
                     let iosystemdnetwork = Server { state: state.clone() };
                     Ok(VarlinkService::new(iosystemdnetwork.get_name().into(),
                                            iosystemdnetwork.get_description().into(),
                                            vec![Box::new(iosystemdnetwork)]))
                 });
}

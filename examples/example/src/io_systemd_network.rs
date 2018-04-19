//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use serde_json::{self, Value};
use std::io;
use std::sync::{Arc, RwLock};
use varlink;
use varlink::CallTrait;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Netdev {
    pub ifindex: i64,
    pub ifname: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NetdevInfo {
    pub ifindex: i64,
    pub ifname: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InfoReply_ {
    pub info: NetdevInfo,
}

impl varlink::VarlinkReply for InfoReply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InfoArgs_ {
    pub ifindex: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ListReply_ {
    pub netdevs: Vec<Netdev>,
}

impl varlink::VarlinkReply for ListReply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ListArgs_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UnknownErrorArgs_ {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UnknownNetworkIfIndexArgs_ {
    pub ifindex: i64,
}

pub trait _CallErr: varlink::CallTrait {
    fn reply_unknown_error(&mut self, text: String) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "io.systemd.network.UnknownError".into(),
            Some(serde_json::to_value(UnknownErrorArgs_ { text }).unwrap()),
        ))
    }
    fn reply_unknown_network_if_index(&mut self, ifindex: i64) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "io.systemd.network.UnknownNetworkIfIndex".into(),
            Some(serde_json::to_value(UnknownNetworkIfIndexArgs_ { ifindex }).unwrap()),
        ))
    }
}

impl<'a> _CallErr for varlink::Call<'a> {}

#[derive(Debug)]
pub enum Error_ {
    UnknownError(Option<UnknownErrorArgs_>),
    UnknownNetworkIfIndex(Option<UnknownNetworkIfIndexArgs_>),
    VarlinkError_(varlink::Error),
    UnknownError_(varlink::Reply),
    IOError_(io::Error),
    JSONError_(serde_json::Error),
}

impl From<varlink::Reply> for Error_ {
    fn from(e: varlink::Reply) -> Self {
        if varlink::Error::is_error(&e) {
            return Error_::VarlinkError_(e.into());
        }

        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "io.systemd.network.UnknownError" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => Error_::UnknownError(v),
                        Err(_) => Error_::UnknownError(None),
                    },
                    _ => Error_::UnknownError(None),
                }
            }
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "io.systemd.network.UnknownNetworkIfIndex" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => Error_::UnknownNetworkIfIndex(v),
                        Err(_) => Error_::UnknownNetworkIfIndex(None),
                    },
                    _ => Error_::UnknownNetworkIfIndex(None),
                }
            }
            _ => return Error_::UnknownError_(e),
        }
    }
}

impl From<io::Error> for Error_ {
    fn from(e: io::Error) -> Self {
        Error_::IOError_(e)
    }
}

impl From<serde_json::Error> for Error_ {
    fn from(e: serde_json::Error) -> Self {
        use serde_json::error::Category;
        match e.classify() {
            Category::Io => Error_::IOError_(e.into()),
            _ => Error_::JSONError_(e),
        }
    }
}

impl From<Error_> for io::Error {
    fn from(e: Error_) -> Self {
        match e {
            Error_::UnknownError(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "io.systemd.network.UnknownError: '{}'",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
            Error_::UnknownNetworkIfIndex(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "io.systemd.network.UnknownNetworkIfIndex: '{}'",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
            Error_::VarlinkError_(e) => e.into(),
            Error_::IOError_(e) => e,
            Error_::JSONError_(e) => e.into(),
            Error_::UnknownError_(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "unknown varlink error: {}",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
        }
    }
}
pub trait _CallInfo: _CallErr {
    fn reply(&mut self, info: NetdevInfo) -> io::Result<()> {
        self.reply_struct(InfoReply_ { info }.into())
    }
}

impl<'a> _CallInfo for varlink::Call<'a> {}

pub trait _CallList: _CallErr {
    fn reply(&mut self, netdevs: Vec<Netdev>) -> io::Result<()> {
        self.reply_struct(ListReply_ { netdevs }.into())
    }
}

impl<'a> _CallList for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn info(&self, call: &mut _CallInfo, ifindex: i64) -> io::Result<()>;
    fn list(&self, call: &mut _CallList) -> io::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> io::Result<()> {
        Ok(())
    }
}

pub trait VarlinkClientInterface {
    fn info(
        &mut self,
        ifindex: i64,
    ) -> io::Result<varlink::MethodCall<InfoArgs_, InfoReply_, Error_>>;
    fn list(&mut self) -> io::Result<varlink::MethodCall<ListArgs_, ListReply_, Error_>>;
}

pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
    more: bool,
    oneway: bool,
}

impl VarlinkClient {
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient {
            connection,
            more: false,
            oneway: false,
        }
    }
    pub fn more(&self) -> Self {
        VarlinkClient {
            connection: self.connection.clone(),
            more: true,
            oneway: false,
        }
    }
    pub fn oneway(&self) -> Self {
        VarlinkClient {
            connection: self.connection.clone(),
            more: false,
            oneway: true,
        }
    }
}

impl VarlinkClientInterface for VarlinkClient {
    fn info(
        &mut self,
        ifindex: i64,
    ) -> io::Result<varlink::MethodCall<InfoArgs_, InfoReply_, Error_>> {
        varlink::MethodCall::<InfoArgs_, InfoReply_, Error_>::call(
            self.connection.clone(),
            "io.systemd.network.Info".into(),
            InfoArgs_ { ifindex },
            self.more,
            self.oneway,
        )
    }
    fn list(&mut self) -> io::Result<varlink::MethodCall<ListArgs_, ListReply_, Error_>> {
        varlink::MethodCall::<ListArgs_, ListReply_, Error_>::call(
            self.connection.clone(),
            "io.systemd.network.List".into(),
            ListArgs_ {},
            self.more,
            self.oneway,
        )
    }
}

pub struct _InterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> _InterfaceProxy {
    _InterfaceProxy { inner }
}

impl varlink::Interface for _InterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#"# Provides information about network state
#
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
method List() -> (netdevs: []Netdev)

error UnknownNetworkIfIndex (ifindex: int)
error UnknownError (text: string)"#
    }

    fn get_name(&self) -> &'static str {
        "io.systemd.network"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> io::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> io::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "io.systemd.network.Info" => {
                if let Some(args) = req.parameters.clone() {
                    let args: InfoArgs_ = serde_json::from_value(args)?;
                    return self.inner.info(call as &mut _CallInfo, args.ifindex);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "io.systemd.network.List" => {
                return self.inner.list(call as &mut _CallList);
            }

            m => {
                return call.reply_method_not_found(String::from(m));
            }
        }
    }
}
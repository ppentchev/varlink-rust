//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use failure::{Backtrace, Context, Fail, ResultExt};
use serde_json::{self, Value};
use std::io;
use std::sync::{Arc, RwLock};
use varlink::{self, CallTrait};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Netdev {
    pub ifindex: i64,
    pub ifname: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NetdevInfo {
    pub ifindex: i64,
    pub ifname: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Info_Reply {
    pub info: NetdevInfo,
}

impl varlink::VarlinkReply for Info_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Info_Args {
    pub ifindex: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct List_Reply {
    pub netdevs: Vec<Netdev>,
}

impl varlink::VarlinkReply for List_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct List_Args {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnknownError_Args {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnknownNetworkIfIndex_Args {
    pub ifindex: i64,
}

pub trait VarlinkCallError: varlink::CallTrait {
    fn reply_unknown_error(&mut self, text: String) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "io.systemd.network.UnknownError",
            Some(serde_json::to_value(UnknownError_Args { text })?),
        ))
    }
    fn reply_unknown_network_if_index(&mut self, ifindex: i64) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "io.systemd.network.UnknownNetworkIfIndex",
            Some(serde_json::to_value(UnknownNetworkIfIndex_Args {
                ifindex,
            })?),
        ))
    }
}

impl<'a> VarlinkCallError for varlink::Call<'a> {}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "IO error")]
    Io_Error(::std::io::ErrorKind),
    #[fail(display = "(De)Serialization Error")]
    SerdeJson_Error(serde_json::error::Category),
    #[fail(display = "Varlink Error")]
    Varlink(varlink::ErrorKind),
    #[fail(display = "Unknown error reply: '{:#?}'", _0)]
    VarlinkReply(varlink::Reply),
    #[fail(display = "io.systemd.network.UnknownError: {:#?}", _0)]
    UnknownError(Option<UnknownError_Args>),
    #[fail(display = "io.systemd.network.UnknownNetworkIfIndex: {:#?}", _0)]
    UnknownNetworkIfIndex(Option<UnknownNetworkIfIndex_Args>),
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.inner.get_context().clone()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Error {
        let kind = e.kind();
        e.context(ErrorKind::Io_Error(kind)).into()
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        let cat = e.classify();
        e.context(ErrorKind::SerdeJson_Error(cat)).into()
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<varlink::Error> for Error {
    fn from(e: varlink::Error) -> Self {
        let kind = e.kind();
        match kind {
            varlink::ErrorKind::Io(kind) => e.context(ErrorKind::Io_Error(kind)).into(),
            varlink::ErrorKind::SerdeJsonSer(cat) => {
                e.context(ErrorKind::SerdeJson_Error(cat)).into()
            }
            kind => e.context(ErrorKind::Varlink(kind)).into(),
        }
    }
}

impl From<varlink::Reply> for Error {
    fn from(e: varlink::Reply) -> Self {
        if varlink::Error::is_error(&e) {
            return varlink::Error::from(e).into();
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
                        Ok(v) => ErrorKind::UnknownError(v).into(),
                        Err(_) => ErrorKind::UnknownError(None).into(),
                    },
                    _ => ErrorKind::UnknownError(None).into(),
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
                        Ok(v) => ErrorKind::UnknownNetworkIfIndex(v).into(),
                        Err(_) => ErrorKind::UnknownNetworkIfIndex(None).into(),
                    },
                    _ => ErrorKind::UnknownNetworkIfIndex(None).into(),
                }
            }
            _ => return ErrorKind::VarlinkReply(e).into(),
        }
    }
}
pub trait Call_Info: VarlinkCallError {
    fn reply(&mut self, info: NetdevInfo) -> varlink::Result<()> {
        self.reply_struct(Info_Reply { info }.into())
    }
}

impl<'a> Call_Info for varlink::Call<'a> {}

pub trait Call_List: VarlinkCallError {
    fn reply(&mut self, netdevs: Vec<Netdev>) -> varlink::Result<()> {
        self.reply_struct(List_Reply { netdevs }.into())
    }
}

impl<'a> Call_List for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn info(&self, call: &mut Call_Info, ifindex: i64) -> varlink::Result<()>;
    fn list(&self, call: &mut Call_List) -> varlink::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> varlink::Result<()> {
        Ok(())
    }
}

pub trait VarlinkClientInterface {
    fn info(&mut self, ifindex: i64) -> varlink::MethodCall<Info_Args, Info_Reply, Error>;
    fn list(&mut self) -> varlink::MethodCall<List_Args, List_Reply, Error>;
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
    fn info(&mut self, ifindex: i64) -> varlink::MethodCall<Info_Args, Info_Reply, Error> {
        varlink::MethodCall::<Info_Args, Info_Reply, Error>::new(
            self.connection.clone(),
            "io.systemd.network.Info",
            Info_Args { ifindex },
        )
    }
    fn list(&mut self) -> varlink::MethodCall<List_Args, List_Reply, Error> {
        varlink::MethodCall::<List_Args, List_Reply, Error>::new(
            self.connection.clone(),
            "io.systemd.network.List",
            List_Args {},
        )
    }
}

pub struct VarlinkInterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> VarlinkInterfaceProxy {
    VarlinkInterfaceProxy { inner }
}

impl varlink::Interface for VarlinkInterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#####################################"# Provides information about network state
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
error UnknownError (text: string)
"#####################################
    }

    fn get_name(&self) -> &'static str {
        "io.systemd.network"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "io.systemd.network.Info" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Info_Args = serde_json::from_value(args)?;
                    return self.inner.info(call as &mut Call_Info, args.ifindex);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "io.systemd.network.List" => {
                return self.inner.list(call as &mut Call_List);
            }

            m => {
                return call.reply_method_not_found(String::from(m));
            }
        }
    }
}

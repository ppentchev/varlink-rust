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
pub struct Interface {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foo: Option<Vec<Option<varlink::StringHashMap<Interface_foo>>>>,
    pub anon: Interface_anon,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MyType {
    pub object: Value,
    #[serde(rename = "enum")]
    pub enum_: MyType_enum,
    #[serde(rename = "struct")]
    pub struct_: MyType_struct,
    pub array: Vec<String>,
    pub dictionary: varlink::StringHashMap<String>,
    pub stringset: varlink::StringHashSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable_array_struct: Option<Vec<MyType_nullable_array_struct>>,
    pub interface: Interface,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct End_Reply {
    pub all_ok: bool,
}

impl varlink::VarlinkReply for End_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct End_Args {
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Start_Reply {
    pub client_id: String,
}

impl varlink::VarlinkReply for Start_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Start_Args {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test01_Reply {
    pub bool: bool,
}

impl varlink::VarlinkReply for Test01_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test01_Args {
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test02_Reply {
    pub int: i64,
}

impl varlink::VarlinkReply for Test02_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test02_Args {
    pub client_id: String,
    pub bool: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test03_Reply {
    pub float: f64,
}

impl varlink::VarlinkReply for Test03_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test03_Args {
    pub client_id: String,
    pub int: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test04_Reply {
    pub string: String,
}

impl varlink::VarlinkReply for Test04_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test04_Args {
    pub client_id: String,
    pub float: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test05_Reply {
    pub bool: bool,
    pub int: i64,
    pub float: f64,
    pub string: String,
}

impl varlink::VarlinkReply for Test05_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test05_Args {
    pub client_id: String,
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test06_Reply {
    #[serde(rename = "struct")]
    pub struct_: Test06_Reply_struct,
}

impl varlink::VarlinkReply for Test06_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test06_Args {
    pub client_id: String,
    pub bool: bool,
    pub int: i64,
    pub float: f64,
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test07_Reply {
    pub map: varlink::StringHashMap<String>,
}

impl varlink::VarlinkReply for Test07_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test07_Args {
    pub client_id: String,
    #[serde(rename = "struct")]
    pub struct_: Test07_Args_struct,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test08_Reply {
    pub set: varlink::StringHashSet,
}

impl varlink::VarlinkReply for Test08_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test08_Args {
    pub client_id: String,
    pub map: varlink::StringHashMap<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test09_Reply {
    pub mytype: MyType,
}

impl varlink::VarlinkReply for Test09_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test09_Args {
    pub client_id: String,
    pub set: varlink::StringHashSet,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test10_Reply {
    pub string: String,
}

impl varlink::VarlinkReply for Test10_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test10_Args {
    pub client_id: String,
    pub mytype: MyType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test11_Reply {}

impl varlink::VarlinkReply for Test11_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test11_Args {
    pub client_id: String,
    pub last_more_replies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CertificationError_Args {
    pub wants: Value,
    pub got: Value,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ClientIdError_Args {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Interface_anon {
    pub foo: bool,
    pub bar: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MyType_struct {
    pub first: i64,
    pub second: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MyType_nullable_array_struct {
    pub first: i64,
    pub second: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test06_Reply_struct {
    pub bool: bool,
    pub int: i64,
    pub float: f64,
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Test07_Args_struct {
    pub bool: bool,
    pub int: i64,
    pub float: f64,
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Interface_foo {
    foo,
    bar,
    baz,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MyType_enum {
    one,
    two,
    three,
}

pub trait VarlinkCallError: varlink::CallTrait {
    fn reply_certification_error(&mut self, wants: Value, got: Value) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.varlink.certification.CertificationError",
            Some(serde_json::to_value(CertificationError_Args {
                wants,
                got,
            })?),
        ))
    }
    fn reply_client_id_error(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.varlink.certification.ClientIdError",
            None,
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
    #[fail(display = "org.varlink.certification.CertificationError: {:#?}", _0)]
    CertificationError(Option<CertificationError_Args>),
    #[fail(display = "org.varlink.certification.ClientIdError: {:#?}", _0)]
    ClientIdError(Option<ClientIdError_Args>),
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
            } if t == "org.varlink.certification.CertificationError" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => ErrorKind::CertificationError(v).into(),
                        Err(_) => ErrorKind::CertificationError(None).into(),
                    },
                    _ => ErrorKind::CertificationError(None).into(),
                }
            }
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.certification.ClientIdError" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => ErrorKind::ClientIdError(v).into(),
                        Err(_) => ErrorKind::ClientIdError(None).into(),
                    },
                    _ => ErrorKind::ClientIdError(None).into(),
                }
            }
            _ => return ErrorKind::VarlinkReply(e).into(),
        }
    }
}
pub trait Call_End: VarlinkCallError {
    fn reply(&mut self, all_ok: bool) -> varlink::Result<()> {
        self.reply_struct(End_Reply { all_ok }.into())
    }
}

impl<'a> Call_End for varlink::Call<'a> {}

pub trait Call_Start: VarlinkCallError {
    fn reply(&mut self, client_id: String) -> varlink::Result<()> {
        self.reply_struct(Start_Reply { client_id }.into())
    }
}

impl<'a> Call_Start for varlink::Call<'a> {}

pub trait Call_Test01: VarlinkCallError {
    fn reply(&mut self, bool: bool) -> varlink::Result<()> {
        self.reply_struct(Test01_Reply { bool }.into())
    }
}

impl<'a> Call_Test01 for varlink::Call<'a> {}

pub trait Call_Test02: VarlinkCallError {
    fn reply(&mut self, int: i64) -> varlink::Result<()> {
        self.reply_struct(Test02_Reply { int }.into())
    }
}

impl<'a> Call_Test02 for varlink::Call<'a> {}

pub trait Call_Test03: VarlinkCallError {
    fn reply(&mut self, float: f64) -> varlink::Result<()> {
        self.reply_struct(Test03_Reply { float }.into())
    }
}

impl<'a> Call_Test03 for varlink::Call<'a> {}

pub trait Call_Test04: VarlinkCallError {
    fn reply(&mut self, string: String) -> varlink::Result<()> {
        self.reply_struct(Test04_Reply { string }.into())
    }
}

impl<'a> Call_Test04 for varlink::Call<'a> {}

pub trait Call_Test05: VarlinkCallError {
    fn reply(&mut self, bool: bool, int: i64, float: f64, string: String) -> varlink::Result<()> {
        self.reply_struct(
            Test05_Reply {
                bool,
                int,
                float,
                string,
            }.into(),
        )
    }
}

impl<'a> Call_Test05 for varlink::Call<'a> {}

pub trait Call_Test06: VarlinkCallError {
    fn reply(&mut self, struct_: Test06_Reply_struct) -> varlink::Result<()> {
        self.reply_struct(Test06_Reply { struct_ }.into())
    }
}

impl<'a> Call_Test06 for varlink::Call<'a> {}

pub trait Call_Test07: VarlinkCallError {
    fn reply(&mut self, map: varlink::StringHashMap<String>) -> varlink::Result<()> {
        self.reply_struct(Test07_Reply { map }.into())
    }
}

impl<'a> Call_Test07 for varlink::Call<'a> {}

pub trait Call_Test08: VarlinkCallError {
    fn reply(&mut self, set: varlink::StringHashSet) -> varlink::Result<()> {
        self.reply_struct(Test08_Reply { set }.into())
    }
}

impl<'a> Call_Test08 for varlink::Call<'a> {}

pub trait Call_Test09: VarlinkCallError {
    fn reply(&mut self, mytype: MyType) -> varlink::Result<()> {
        self.reply_struct(Test09_Reply { mytype }.into())
    }
}

impl<'a> Call_Test09 for varlink::Call<'a> {}

pub trait Call_Test10: VarlinkCallError {
    fn reply(&mut self, string: String) -> varlink::Result<()> {
        self.reply_struct(Test10_Reply { string }.into())
    }
}

impl<'a> Call_Test10 for varlink::Call<'a> {}

pub trait Call_Test11: VarlinkCallError {
    fn reply(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}

impl<'a> Call_Test11 for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn end(&self, call: &mut Call_End, client_id: String) -> varlink::Result<()>;
    fn start(&self, call: &mut Call_Start) -> varlink::Result<()>;
    fn test01(&self, call: &mut Call_Test01, client_id: String) -> varlink::Result<()>;
    fn test02(&self, call: &mut Call_Test02, client_id: String, bool: bool) -> varlink::Result<()>;
    fn test03(&self, call: &mut Call_Test03, client_id: String, int: i64) -> varlink::Result<()>;
    fn test04(&self, call: &mut Call_Test04, client_id: String, float: f64) -> varlink::Result<()>;
    fn test05(
        &self,
        call: &mut Call_Test05,
        client_id: String,
        string: String,
    ) -> varlink::Result<()>;
    fn test06(
        &self,
        call: &mut Call_Test06,
        client_id: String,
        bool: bool,
        int: i64,
        float: f64,
        string: String,
    ) -> varlink::Result<()>;
    fn test07(
        &self,
        call: &mut Call_Test07,
        client_id: String,
        struct_: Test07_Args_struct,
    ) -> varlink::Result<()>;
    fn test08(
        &self,
        call: &mut Call_Test08,
        client_id: String,
        map: varlink::StringHashMap<String>,
    ) -> varlink::Result<()>;
    fn test09(
        &self,
        call: &mut Call_Test09,
        client_id: String,
        set: varlink::StringHashSet,
    ) -> varlink::Result<()>;
    fn test10(
        &self,
        call: &mut Call_Test10,
        client_id: String,
        mytype: MyType,
    ) -> varlink::Result<()>;
    fn test11(
        &self,
        call: &mut Call_Test11,
        client_id: String,
        last_more_replies: Vec<String>,
    ) -> varlink::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> varlink::Result<()> {
        Ok(())
    }
}

pub trait VarlinkClientInterface {
    fn end(&mut self, client_id: String) -> varlink::MethodCall<End_Args, End_Reply, Error>;
    fn start(&mut self) -> varlink::MethodCall<Start_Args, Start_Reply, Error>;
    fn test01(
        &mut self,
        client_id: String,
    ) -> varlink::MethodCall<Test01_Args, Test01_Reply, Error>;
    fn test02(
        &mut self,
        client_id: String,
        bool: bool,
    ) -> varlink::MethodCall<Test02_Args, Test02_Reply, Error>;
    fn test03(
        &mut self,
        client_id: String,
        int: i64,
    ) -> varlink::MethodCall<Test03_Args, Test03_Reply, Error>;
    fn test04(
        &mut self,
        client_id: String,
        float: f64,
    ) -> varlink::MethodCall<Test04_Args, Test04_Reply, Error>;
    fn test05(
        &mut self,
        client_id: String,
        string: String,
    ) -> varlink::MethodCall<Test05_Args, Test05_Reply, Error>;
    fn test06(
        &mut self,
        client_id: String,
        bool: bool,
        int: i64,
        float: f64,
        string: String,
    ) -> varlink::MethodCall<Test06_Args, Test06_Reply, Error>;
    fn test07(
        &mut self,
        client_id: String,
        struct_: Test07_Args_struct,
    ) -> varlink::MethodCall<Test07_Args, Test07_Reply, Error>;
    fn test08(
        &mut self,
        client_id: String,
        map: varlink::StringHashMap<String>,
    ) -> varlink::MethodCall<Test08_Args, Test08_Reply, Error>;
    fn test09(
        &mut self,
        client_id: String,
        set: varlink::StringHashSet,
    ) -> varlink::MethodCall<Test09_Args, Test09_Reply, Error>;
    fn test10(
        &mut self,
        client_id: String,
        mytype: MyType,
    ) -> varlink::MethodCall<Test10_Args, Test10_Reply, Error>;
    fn test11(
        &mut self,
        client_id: String,
        last_more_replies: Vec<String>,
    ) -> varlink::MethodCall<Test11_Args, Test11_Reply, Error>;
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
    fn end(&mut self, client_id: String) -> varlink::MethodCall<End_Args, End_Reply, Error> {
        varlink::MethodCall::<End_Args, End_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.End",
            End_Args { client_id },
        )
    }
    fn start(&mut self) -> varlink::MethodCall<Start_Args, Start_Reply, Error> {
        varlink::MethodCall::<Start_Args, Start_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Start",
            Start_Args {},
        )
    }
    fn test01(
        &mut self,
        client_id: String,
    ) -> varlink::MethodCall<Test01_Args, Test01_Reply, Error> {
        varlink::MethodCall::<Test01_Args, Test01_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test01",
            Test01_Args { client_id },
        )
    }
    fn test02(
        &mut self,
        client_id: String,
        bool: bool,
    ) -> varlink::MethodCall<Test02_Args, Test02_Reply, Error> {
        varlink::MethodCall::<Test02_Args, Test02_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test02",
            Test02_Args { client_id, bool },
        )
    }
    fn test03(
        &mut self,
        client_id: String,
        int: i64,
    ) -> varlink::MethodCall<Test03_Args, Test03_Reply, Error> {
        varlink::MethodCall::<Test03_Args, Test03_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test03",
            Test03_Args { client_id, int },
        )
    }
    fn test04(
        &mut self,
        client_id: String,
        float: f64,
    ) -> varlink::MethodCall<Test04_Args, Test04_Reply, Error> {
        varlink::MethodCall::<Test04_Args, Test04_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test04",
            Test04_Args { client_id, float },
        )
    }
    fn test05(
        &mut self,
        client_id: String,
        string: String,
    ) -> varlink::MethodCall<Test05_Args, Test05_Reply, Error> {
        varlink::MethodCall::<Test05_Args, Test05_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test05",
            Test05_Args { client_id, string },
        )
    }
    fn test06(
        &mut self,
        client_id: String,
        bool: bool,
        int: i64,
        float: f64,
        string: String,
    ) -> varlink::MethodCall<Test06_Args, Test06_Reply, Error> {
        varlink::MethodCall::<Test06_Args, Test06_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test06",
            Test06_Args {
                client_id,
                bool,
                int,
                float,
                string,
            },
        )
    }
    fn test07(
        &mut self,
        client_id: String,
        struct_: Test07_Args_struct,
    ) -> varlink::MethodCall<Test07_Args, Test07_Reply, Error> {
        varlink::MethodCall::<Test07_Args, Test07_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test07",
            Test07_Args { client_id, struct_ },
        )
    }
    fn test08(
        &mut self,
        client_id: String,
        map: varlink::StringHashMap<String>,
    ) -> varlink::MethodCall<Test08_Args, Test08_Reply, Error> {
        varlink::MethodCall::<Test08_Args, Test08_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test08",
            Test08_Args { client_id, map },
        )
    }
    fn test09(
        &mut self,
        client_id: String,
        set: varlink::StringHashSet,
    ) -> varlink::MethodCall<Test09_Args, Test09_Reply, Error> {
        varlink::MethodCall::<Test09_Args, Test09_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test09",
            Test09_Args { client_id, set },
        )
    }
    fn test10(
        &mut self,
        client_id: String,
        mytype: MyType,
    ) -> varlink::MethodCall<Test10_Args, Test10_Reply, Error> {
        varlink::MethodCall::<Test10_Args, Test10_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test10",
            Test10_Args { client_id, mytype },
        )
    }
    fn test11(
        &mut self,
        client_id: String,
        last_more_replies: Vec<String>,
    ) -> varlink::MethodCall<Test11_Args, Test11_Reply, Error> {
        varlink::MethodCall::<Test11_Args, Test11_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.certification.Test11",
            Test11_Args {
                client_id,
                last_more_replies,
            },
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
        r#####################################"# Interface to test varlink implementations against.
# First you write a varlink client calling:
# Start, Test01, Test02, …, Test09, End
# The return value of the previous call should be the argument of the next call.
# Then you test this client against well known servers like python or rust from
# https://github.com/varlink/
#
# Next you write a varlink server providing the same service as the well known ones.
# Now run your client against it and run well known clients like python or rust
# from https://github.com/varlink/ against your server. If all works out, then
# your new language bindings should be varlink certified.
interface org.varlink.certification

type Interface (
  foo: ?[]?[string](foo, bar, baz),
  anon: (foo: bool, bar: bool)
)

type MyType (
  object: object,
  enum: (one, two, three),
  struct: (first: int, second: string),
  array: []string,
  dictionary: [string]string,
  stringset: [string](),
  nullable: ?string,
  nullable_array_struct: ?[](first: int, second: string),
  interface: Interface
)

method Start() -> (client_id: string)

method Test01(client_id: string) -> (bool: bool)

method Test02(client_id: string, bool: bool) -> (int: int)

method Test03(client_id: string, int: int) -> (float: float)

method Test04(client_id: string, float: float) -> (string: string)

method Test05(client_id: string, string: string) -> (
  bool: bool,
  int: int,
  float: float,
  string: string
)

method Test06(
  client_id: string,
  bool: bool,
  int: int,
  float: float,
  string: string
) -> (
  struct: (
    bool: bool,
    int: int,
    float: float,
    string: string
  )
)

method Test07(
  client_id: string,
  struct: (
    bool: bool,
    int: int,
    float: float,
    string: string
  )
) -> (map: [string]string)

method Test08(client_id: string, map: [string]string) -> (set: [string]())

method Test09(client_id: string, set: [string]()) -> (mytype: MyType)

# returns more than one reply with "continues"
method Test10(client_id: string, mytype: MyType) -> (string: string)

# must be called as "oneway"
method Test11(client_id: string, last_more_replies: []string) -> ()

method End(client_id: string) -> (all_ok: bool)

error ClientIdError ()
error CertificationError (wants: object, got: object)
"#####################################
    }

    fn get_name(&self) -> &'static str {
        "org.varlink.certification"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.varlink.certification.End" => {
                if let Some(args) = req.parameters.clone() {
                    let args: End_Args = serde_json::from_value(args)?;
                    return self.inner.end(call as &mut Call_End, args.client_id);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Start" => {
                return self.inner.start(call as &mut Call_Start);
            }
            "org.varlink.certification.Test01" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test01_Args = serde_json::from_value(args)?;
                    return self.inner.test01(call as &mut Call_Test01, args.client_id);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test02" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test02_Args = serde_json::from_value(args)?;
                    return self
                        .inner
                        .test02(call as &mut Call_Test02, args.client_id, args.bool);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test03" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test03_Args = serde_json::from_value(args)?;
                    return self
                        .inner
                        .test03(call as &mut Call_Test03, args.client_id, args.int);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test04" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test04_Args = serde_json::from_value(args)?;
                    return self
                        .inner
                        .test04(call as &mut Call_Test04, args.client_id, args.float);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test05" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test05_Args = serde_json::from_value(args)?;
                    return self
                        .inner
                        .test05(call as &mut Call_Test05, args.client_id, args.string);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test06" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test06_Args = serde_json::from_value(args)?;
                    return self.inner.test06(
                        call as &mut Call_Test06,
                        args.client_id,
                        args.bool,
                        args.int,
                        args.float,
                        args.string,
                    );
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test07" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test07_Args = serde_json::from_value(args)?;
                    return self.inner.test07(
                        call as &mut Call_Test07,
                        args.client_id,
                        args.struct_,
                    );
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test08" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test08_Args = serde_json::from_value(args)?;
                    return self
                        .inner
                        .test08(call as &mut Call_Test08, args.client_id, args.map);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test09" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test09_Args = serde_json::from_value(args)?;
                    return self
                        .inner
                        .test09(call as &mut Call_Test09, args.client_id, args.set);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test10" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test10_Args = serde_json::from_value(args)?;
                    return self
                        .inner
                        .test10(call as &mut Call_Test10, args.client_id, args.mytype);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test11" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test11_Args = serde_json::from_value(args)?;
                    return self.inner.test11(
                        call as &mut Call_Test11,
                        args.client_id,
                        args.last_more_replies,
                    );
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }

            m => {
                return call.reply_method_not_found(String::from(m));
            }
        }
    }
}

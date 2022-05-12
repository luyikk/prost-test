#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoneNumber {
    #[prost(string, tag = "1")]
    pub number: ::prost::alloc::string::String,
    #[prost(enumeration = "PhoneType", tag = "2")]
    pub r#type: i32,
}

#[allow(dead_code)]
pub const PHONE_NUMBER_ID: i32 = PhoneNumber::get_msg_id();

impl PhoneNumber {
    #[allow(dead_code)]
    pub const fn get_msg_id() -> i32 {
        150000
    }
}

impl ::prost_msg_id::MsgId for PhoneNumber {
    fn get_msg_id(&self) -> i32 {
        Self::get_msg_id()
    }
}

impl TryFrom<&[u8]> for PhoneNumber {
    type Error = ::prost::DecodeError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        ::prost::Message::decode(value)
    }
}
impl TryFrom<Vec<u8>> for PhoneNumber {
    type Error = ::prost::DecodeError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        ::prost::Message::decode(value.as_slice())
    }
}
/// Nested message and enum types in `PhoneNumber`.
pub mod phone_number {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgId {
        None = 0,
        Id = 150000,
    }
}
/// aaaa
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Foo {
    /// bbb
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// ccc
    #[prost(int32, tag = "2")]
    pub id: i32,
    /// xxxx
    #[prost(message, repeated, tag = "4")]
    pub phones: ::prost::alloc::vec::Vec<PhoneNumber>,
}

/// aaaa
#[allow(dead_code)]
pub const FOO_ID: i32 = Foo::get_msg_id();

impl Foo {
    #[allow(dead_code)]
    pub const fn get_msg_id() -> i32 {
        150001
    }
}

impl ::prost_msg_id::MsgId for Foo {
    fn get_msg_id(&self) -> i32 {
        Self::get_msg_id()
    }
}

impl TryFrom<&[u8]> for Foo {
    type Error = ::prost::DecodeError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        ::prost::Message::decode(value)
    }
}
impl TryFrom<Vec<u8>> for Foo {
    type Error = ::prost::DecodeError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        ::prost::Message::decode(value.as_slice())
    }
}
/// Nested message and enum types in `Foo`.
pub mod foo {
    ///通用返回失败
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fail {
        #[prost(int64, tag = "1")]
        pub number: i64,
        #[prost(string, tag = "2")]
        pub message: ::prost::alloc::string::String,
    }

    ///通用返回失败
    #[allow(dead_code)]
    pub const FAIL_ID: i32 = Fail::get_msg_id();

    impl Fail {
        #[allow(dead_code)]
        pub const fn get_msg_id() -> i32 {
            150004
        }
    }

    impl ::prost_msg_id::MsgId for Fail {
        fn get_msg_id(&self) -> i32 {
            Self::get_msg_id()
        }
    }

    impl TryFrom<&[u8]> for Fail {
        type Error = ::prost::DecodeError;
        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            ::prost::Message::decode(value)
        }
    }
    impl TryFrom<Vec<u8>> for Fail {
        type Error = ::prost::DecodeError;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            ::prost::Message::decode(value.as_slice())
        }
    }
    /// Nested message and enum types in `Fail`.
    pub mod fail {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum MsgId {
            None = 0,
            Id = 150004,
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgId {
        None = 0,
        Id = 150001,
    }
}
///通用返回失败
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fail {
    #[prost(int64, tag = "1")]
    pub number: i64,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}

///通用返回失败
#[allow(dead_code)]
pub const FAIL_ID: i32 = Fail::get_msg_id();

impl Fail {
    #[allow(dead_code)]
    pub const fn get_msg_id() -> i32 {
        150002
    }
}

impl ::prost_msg_id::MsgId for Fail {
    fn get_msg_id(&self) -> i32 {
        Self::get_msg_id()
    }
}

impl TryFrom<&[u8]> for Fail {
    type Error = ::prost::DecodeError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        ::prost::Message::decode(value)
    }
}
impl TryFrom<Vec<u8>> for Fail {
    type Error = ::prost::DecodeError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        ::prost::Message::decode(value.as_slice())
    }
}
/// Nested message and enum types in `Fail`.
pub mod fail {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgId {
        None = 0,
        Id = 150002,
    }
}
///通用错误返回
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(int64, tag = "1")]
    pub number: i64,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}

///通用错误返回
#[allow(dead_code)]
pub const ERROR_ID: i32 = Error::get_msg_id();

impl Error {
    #[allow(dead_code)]
    pub const fn get_msg_id() -> i32 {
        150003
    }
}

impl ::prost_msg_id::MsgId for Error {
    fn get_msg_id(&self) -> i32 {
        Self::get_msg_id()
    }
}

impl TryFrom<&[u8]> for Error {
    type Error = ::prost::DecodeError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        ::prost::Message::decode(value)
    }
}
impl TryFrom<Vec<u8>> for Error {
    type Error = ::prost::DecodeError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        ::prost::Message::decode(value.as_slice())
    }
}
/// Nested message and enum types in `Error`.
pub mod error {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgId {
        None = 0,
        Id = 150003,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhoneType {
    Mobile = 0,
    ///AVV
    Home = 1,
    Work = 2,
}

///get all msg type id
#[allow(dead_code)]
pub const fn msg_ids()->&'static [i32]{
    &[
        150001, //.RunTT.Foo
        150002, //.RunTT.Fail
        150000, //.RunTT.PhoneNumber
        150003, //.RunTT.Error
        150004, //.RunTT.Foo.Fail
    ]
}
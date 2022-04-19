#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoneNumber {
    #[prost(string, tag="1")]
    pub number: ::prost::alloc::string::String,
    #[prost(enumeration="PhoneType", tag="2")]
    pub r#type: i32,
}
/// aaaa
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Foo {
    /// bbb
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// ccc
    #[prost(int32, tag="2")]
    pub id: i32,
    /// xxxx
    #[prost(message, repeated, tag="4")]
    pub phones: ::prost::alloc::vec::Vec<PhoneNumber>,
}
/// Nested message and enum types in `Foo`.
pub mod foo {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgId {
        None = 0,
        Id = 123,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhoneType {
    Mobile = 0,
    Home = 1,
    Work = 2,
}

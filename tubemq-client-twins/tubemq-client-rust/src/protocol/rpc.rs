#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcConnHeader {
    #[prost(int32, required, tag = "1")]
    pub flag: i32,
    #[prost(int64, optional, tag = "2")]
    pub trace_id: ::std::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub span_id: ::std::option::Option<i64>,
    #[prost(int64, optional, tag = "4")]
    pub parent_id: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestHeader {
    #[prost(int32, optional, tag = "1")]
    pub service_type: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub protocol_ver: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBody {
    #[prost(int32, required, tag = "1")]
    pub method: i32,
    #[prost(int64, optional, tag = "2")]
    pub timeout: ::std::option::Option<i64>,
    #[prost(bytes, optional, tag = "3")]
    pub request: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseHeader {
    #[prost(enumeration = "response_header::Status", required, tag = "1")]
    pub status: i32,
    #[prost(int32, optional, tag = "2")]
    pub service_type: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub protocol_ver: ::std::option::Option<i32>,
}
pub mod response_header {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Success = 0,
        Error = 1,
        Fatal = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspExceptionBody {
    #[prost(string, required, tag = "1")]
    pub exception_name: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub stack_trace: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspResponseBody {
    #[prost(int32, required, tag = "1")]
    pub method: i32,
    #[prost(bytes, required, tag = "2")]
    pub data: std::vec::Vec<u8>,
}

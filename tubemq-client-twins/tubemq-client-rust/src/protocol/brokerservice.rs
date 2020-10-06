#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferedMessage {
    #[prost(int64, required, tag = "1")]
    pub message_id: i64,
    #[prost(int32, required, tag = "2")]
    pub check_sum: i32,
    #[prost(bytes, required, tag = "3")]
    pub pay_load_data: std::vec::Vec<u8>,
    #[prost(int32, required, tag = "4")]
    pub flag: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedInfo {
    #[prost(int64, required, tag = "1")]
    pub visit_authorized_token: i64,
    #[prost(string, optional, tag = "2")]
    pub auth_authorized_token: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageRequestP2b {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub topic_name: std::string::String,
    #[prost(int32, required, tag = "3")]
    pub partition_id: i32,
    #[prost(bytes, required, tag = "4")]
    pub data: std::vec::Vec<u8>,
    #[prost(int32, required, tag = "5")]
    pub flag: i32,
    #[prost(int32, required, tag = "6")]
    pub check_sum: i32,
    #[prost(int32, required, tag = "7")]
    pub sent_addr: i32,
    #[prost(string, optional, tag = "8")]
    pub msg_type: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "9")]
    pub msg_time: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "10")]
    pub auth_info: ::std::option::Option<AuthorizedInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageResponseB2p {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
    #[prost(bool, optional, tag = "4")]
    pub require_auth: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequestC2b {
    #[prost(int32, required, tag = "1")]
    pub op_type: i32,
    #[prost(string, required, tag = "2")]
    pub client_id: std::string::String,
    #[prost(string, required, tag = "3")]
    pub group_name: std::string::String,
    #[prost(string, required, tag = "4")]
    pub topic_name: std::string::String,
    #[prost(int32, required, tag = "5")]
    pub partition_id: i32,
    #[prost(int32, required, tag = "6")]
    pub read_status: i32,
    #[prost(string, repeated, tag = "7")]
    pub filter_cond_str: ::std::vec::Vec<std::string::String>,
    #[prost(int64, optional, tag = "8")]
    pub curr_offset: ::std::option::Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub session_key: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag = "10")]
    pub session_time: ::std::option::Option<i64>,
    /// Deprecated  
    #[prost(int64, optional, tag = "11")]
    pub ssd_store_id: ::std::option::Option<i64>,
    #[prost(int32, optional, tag = "12")]
    pub qry_priority_id: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "13")]
    pub auth_info: ::std::option::Option<AuthorizedInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponseB2c {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
    #[prost(int64, optional, tag = "4")]
    pub curr_offset: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartBeatRequestC2b {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub group_name: std::string::String,
    #[prost(int32, required, tag = "3")]
    pub read_status: i32,
    /// brokerId:host:port:topic:partitionId:delayTimeStamp
    #[prost(string, repeated, tag = "4")]
    pub partition_info: ::std::vec::Vec<std::string::String>,
    /// Deprecated  
    #[prost(int64, optional, tag = "5")]
    pub ssd_store_id: ::std::option::Option<i64>,
    #[prost(int32, optional, tag = "6")]
    pub qry_priority_id: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "7")]
    pub auth_info: ::std::option::Option<AuthorizedInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartBeatResponseB2c {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
    #[prost(bool, optional, tag = "4")]
    pub has_part_failure: ::std::option::Option<bool>,
    /// failCode:brokerId:host:port:topic:partitionId:delayTimeStamp
    #[prost(string, repeated, tag = "5")]
    pub failure_info: ::std::vec::Vec<std::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub require_auth: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageRequestC2b {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(int32, required, tag = "2")]
    pub partition_id: i32,
    #[prost(string, required, tag = "3")]
    pub group_name: std::string::String,
    #[prost(string, required, tag = "4")]
    pub topic_name: std::string::String,
    #[prost(bool, optional, tag = "5")]
    pub last_pack_consumed: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub manual_commit_offset: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub esc_flow_ctrl: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageResponseB2c {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, optional, tag = "3")]
    pub err_msg: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub messages: ::std::vec::Vec<TransferedMessage>,
    #[prost(int64, optional, tag = "5")]
    pub curr_offset: ::std::option::Option<i64>,
    #[prost(int32, optional, tag = "6")]
    pub min_limit_time: ::std::option::Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub esc_flow_ctrl: ::std::option::Option<bool>,
    #[prost(int64, optional, tag = "8")]
    pub curr_data_dlt: ::std::option::Option<i64>,
    #[prost(bool, optional, tag = "9")]
    pub require_slow: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitOffsetRequestC2b {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub topic_name: std::string::String,
    #[prost(int32, required, tag = "3")]
    pub partition_id: i32,
    #[prost(string, required, tag = "4")]
    pub group_name: std::string::String,
    #[prost(bool, optional, tag = "5")]
    pub last_pack_consumed: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitOffsetResponseB2c {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
    #[prost(int64, optional, tag = "4")]
    pub curr_offset: ::std::option::Option<i64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventProto {
    #[prost(int64, optional, tag = "1")]
    pub rebalance_id: ::std::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub op_type: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub status: ::std::option::Option<i32>,
    /// consumerId@group-brokerId:host:port-topic:partitionId
    #[prost(string, repeated, tag = "4")]
    pub subscribe_info: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableBrokerFunInfo {
    #[prost(bool, required, tag = "1")]
    pub enable_consume_authenticate: bool,
    #[prost(bool, required, tag = "2")]
    pub enable_consume_authorize: bool,
    #[prost(bool, required, tag = "3")]
    pub enable_produce_authenticate: bool,
    #[prost(bool, required, tag = "4")]
    pub enable_produce_authorize: bool,
    #[prost(bool, optional, tag = "5")]
    pub enable_visit_token_check: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateInfo {
    #[prost(string, required, tag = "1")]
    pub user_name: std::string::String,
    #[prost(int64, required, tag = "2")]
    pub timestamp: i64,
    #[prost(int32, required, tag = "3")]
    pub nonce: i32,
    #[prost(string, required, tag = "4")]
    pub oth_params: std::string::String,
    #[prost(string, required, tag = "5")]
    pub signature: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterCertificateInfo {
    #[prost(message, optional, tag = "1")]
    pub auth_info: ::std::option::Option<AuthenticateInfo>,
    #[prost(string, optional, tag = "2")]
    pub authorized_token: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterAuthorizedInfo {
    #[prost(int64, required, tag = "1")]
    pub visit_authorized_token: i64,
    #[prost(string, optional, tag = "2")]
    pub auth_authorized_token: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterBrokerAuthorizedInfo {
    #[prost(string, required, tag = "1")]
    pub visit_authorized_token: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub auth_authorized_token: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequestP2m {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(string, repeated, tag = "2")]
    pub topic_list: ::std::vec::Vec<std::string::String>,
    #[prost(int64, required, tag = "3")]
    pub broker_check_sum: i64,
    #[prost(string, required, tag = "4")]
    pub host_name: std::string::String,
    #[prost(message, optional, tag = "5")]
    pub auth_info: ::std::option::Option<MasterCertificateInfo>,
    #[prost(string, optional, tag = "6")]
    pub jdk_version: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponseM2p {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
    #[prost(int64, required, tag = "4")]
    pub broker_check_sum: i64,
    #[prost(string, repeated, tag = "5")]
    pub broker_infos: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag = "6")]
    pub authorized_info: ::std::option::Option<MasterAuthorizedInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartRequestP2m {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(int64, required, tag = "2")]
    pub broker_check_sum: i64,
    #[prost(string, required, tag = "3")]
    pub host_name: std::string::String,
    #[prost(string, repeated, tag = "4")]
    pub topic_list: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag = "5")]
    pub auth_info: ::std::option::Option<MasterCertificateInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartResponseM2p {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
    #[prost(int64, required, tag = "4")]
    pub broker_check_sum: i64,
    /// brokerId:host:port-topic:partitionNum
    #[prost(string, repeated, tag = "5")]
    pub topic_infos: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub broker_infos: ::std::vec::Vec<std::string::String>,
    #[prost(bool, optional, tag = "7")]
    pub require_auth: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "8")]
    pub authorized_info: ::std::option::Option<MasterAuthorizedInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseRequestP2m {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub auth_info: ::std::option::Option<MasterCertificateInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseResponseM2p {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequestC2m {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub group_name: std::string::String,
    #[prost(string, required, tag = "3")]
    pub host_name: std::string::String,
    #[prost(string, repeated, tag = "4")]
    pub topic_list: ::std::vec::Vec<std::string::String>,
    /// consumerId@group-brokerId:host:port-topic:partitionId
    #[prost(string, repeated, tag = "5")]
    pub subscribe_info: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub topic_condition: ::std::vec::Vec<std::string::String>,
    #[prost(bool, optional, tag = "7")]
    pub require_bound: ::std::option::Option<bool>,
    #[prost(int64, optional, tag = "8")]
    pub session_time: ::std::option::Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub session_key: ::std::option::Option<std::string::String>,
    #[prost(int32, optional, tag = "10")]
    pub total_count: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub required_partition: ::std::option::Option<std::string::String>,
    #[prost(bool, optional, tag = "12")]
    pub not_allocated: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "13")]
    pub select_big: ::std::option::Option<bool>,
    #[prost(int64, optional, tag = "14")]
    pub group_flow_check_id: ::std::option::Option<i64>,
    #[prost(int64, optional, tag = "15")]
    pub def_flow_check_id: ::std::option::Option<i64>,
    /// Deprecated  
    #[prost(int64, optional, tag = "16")]
    pub ssd_store_id: ::std::option::Option<i64>,
    #[prost(int32, optional, tag = "17")]
    pub qry_priority_id: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "18")]
    pub auth_info: ::std::option::Option<MasterCertificateInfo>,
    #[prost(string, optional, tag = "19")]
    pub jdk_version: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponseM2c {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
    /// brokerId:host:port-topic:partitionNum
    #[prost(string, repeated, tag = "4")]
    pub topic_info: ::std::vec::Vec<std::string::String>,
    #[prost(bool, optional, tag = "5")]
    pub not_allocated: ::std::option::Option<bool>,
    #[prost(int64, optional, tag = "6")]
    pub def_flow_check_id: ::std::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub def_flow_control_info: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag = "8")]
    pub group_flow_check_id: ::std::option::Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub group_flow_control_info: ::std::option::Option<std::string::String>,
    /// Deprecated  
    #[prost(int64, optional, tag = "10")]
    pub ssd_store_id: ::std::option::Option<i64>,
    #[prost(int32, optional, tag = "11")]
    pub qry_priority_id: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "12")]
    pub authorized_info: ::std::option::Option<MasterAuthorizedInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartRequestC2m {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub group_name: std::string::String,
    #[prost(string, repeated, tag = "3")]
    pub subscribe_info: ::std::vec::Vec<std::string::String>,
    #[prost(bool, required, tag = "4")]
    pub report_subscribe_info: bool,
    #[prost(message, optional, tag = "5")]
    pub event: ::std::option::Option<EventProto>,
    #[prost(int64, optional, tag = "6")]
    pub def_flow_check_id: ::std::option::Option<i64>,
    #[prost(int64, optional, tag = "7")]
    pub group_flow_check_id: ::std::option::Option<i64>,
    /// Deprecated  
    #[prost(int64, optional, tag = "8")]
    pub ssd_store_id: ::std::option::Option<i64>,
    #[prost(int32, optional, tag = "9")]
    pub qry_priority_id: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "10")]
    pub auth_info: ::std::option::Option<MasterCertificateInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartResponseM2c {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub event: ::std::option::Option<EventProto>,
    #[prost(bool, optional, tag = "5")]
    pub not_allocated: ::std::option::Option<bool>,
    #[prost(int64, optional, tag = "6")]
    pub def_flow_check_id: ::std::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub def_flow_control_info: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag = "8")]
    pub group_flow_check_id: ::std::option::Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub group_flow_control_info: ::std::option::Option<std::string::String>,
    /// Deprecated  
    #[prost(int64, optional, tag = "10")]
    pub ssd_store_id: ::std::option::Option<i64>,
    #[prost(int32, optional, tag = "11")]
    pub qry_priority_id: ::std::option::Option<i32>,
    #[prost(bool, optional, tag = "12")]
    pub require_auth: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "13")]
    pub authorized_info: ::std::option::Option<MasterAuthorizedInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseRequestC2m {
    #[prost(string, required, tag = "1")]
    pub client_id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub group_name: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub auth_info: ::std::option::Option<MasterCertificateInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseResponseM2c {
    #[prost(bool, required, tag = "1")]
    pub success: bool,
    #[prost(int32, required, tag = "2")]
    pub err_code: i32,
    #[prost(string, required, tag = "3")]
    pub err_msg: std::string::String,
}

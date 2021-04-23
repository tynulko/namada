#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
    #[prost(bytes = "vec", tag = "1")]
    pub code: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTopic {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentBroadcasterMessage {
    #[prost(
        oneof = "intent_broadcaster_message::IntentMessage",
        tags = "1, 2"
    )]
    pub intent_message:
        ::core::option::Option<intent_broadcaster_message::IntentMessage>,
}
/// Nested message and enum types in `IntentBroadcasterMessage`.
pub mod intent_broadcaster_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum IntentMessage {
        #[prost(message, tag = "1")]
        Intent(super::Intent),
        #[prost(message, tag = "2")]
        SubscribeTopic(super::SubscribeTopic),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dkg {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgBroadcasterMessage {
    #[prost(oneof = "dkg_broadcaster_message::DkgMessage", tags = "1")]
    pub dkg_message:
        ::core::option::Option<dkg_broadcaster_message::DkgMessage>,
}
/// Nested message and enum types in `DkgBroadcasterMessage`.
pub mod dkg_broadcaster_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DkgMessage {
        #[prost(message, tag = "1")]
        Dkg(super::Dkg),
    }
}

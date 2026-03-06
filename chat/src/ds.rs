use std::sync::mpsc;

#[derive(Debug)]
pub enum DeliveryServiceError {
    WakuNodeAlreadyInitialized(String),
    WakuPublishMessageError(String),
    Other(anyhow::Error),
}

impl std::fmt::Display for DeliveryServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WakuNodeAlreadyInitialized(s) => write!(f, "waku node already initialized: {s}"),
            Self::WakuPublishMessageError(s) => write!(f, "waku publish error: {s}"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for DeliveryServiceError {}

pub mod transport {
    use std::sync::mpsc;

    use super::DeliveryServiceError;

    #[derive(Clone)]
    pub struct InboundPacket {
        pub payload: Vec<u8>,
        pub subtopic: String,
        pub group_id: String,
        pub app_id: Vec<u8>,
        pub timestamp: i64,
    }

    pub struct OutboundPacket {
        pub group_id: String,
        pub subtopic: String,
        pub payload: Vec<u8>,
        pub app_id: Vec<u8>,
    }

    pub trait DeliveryService {
        fn send(&self, pkt: OutboundPacket) -> Result<String, DeliveryServiceError>;
        fn subscribe(&self) -> mpsc::Receiver<InboundPacket>;
    }
}

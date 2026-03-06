mod ds;
mod waku;

use ds::transport::DeliveryService;
use waku::{WakuConfig, WakuDeliveryService};

const GROUP_VERSION: &str = "1";
const SUBTOPICS: &[&str] = &["chat"];

fn main() {
    let result = WakuDeliveryService::start(WakuConfig::default())
        .expect("failed to start waku node");

    println!("Waku node started. Listening for messages...");

    let rx = result.service.subscribe();
    for pkt in rx {
        println!(
            "--- received {} bytes (group={}, subtopic={}) ---",
            pkt.payload.len(),
            pkt.group_id,
            pkt.subtopic
        );
        println!("raw bytes: {:?}", pkt.payload);
    }
}

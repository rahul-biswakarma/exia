mod core;
use core::network::scanner::scan_arp_async;

#[tokio::main]
async fn main() {
    let devices = scan_arp_async().await;
    for d in devices {
        println!("{:?}", d);
    }
}

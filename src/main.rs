mod core;

use core::network::scanner::scan_local_network_interfaces;

fn main() {
    let interfaces = scan_local_network_interfaces();
    for i in interfaces {
        println!("Interface: {:?}", i);
    }
}

mod core;

use core::network::scanner::scan_local_network_devices;

fn main() {
    let interfaces = scan_local_network_devices();
    for i in interfaces {
        println!(
            "Interface: {:?} {:?} {:?} {:?}",
            i.device_name, i.vendor, i.ip_address, i.mac_address
        );
    }
}

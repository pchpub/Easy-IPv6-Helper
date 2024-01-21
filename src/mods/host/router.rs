use crate::mods::network::ipv6::format_ipv6;
use std::process::Command;

pub fn config_router(interface: &str, address: &[u16; 8], prefix_length: u8) {
    let _ = Command::new("ip")
        .arg("-6")
        .arg("route")
        .arg("del")
        .arg(format!("{}/{}", format_ipv6(address), prefix_length))
        .arg("dev")
        .arg(interface)
        .output();

    let _ = Command::new("ip")
        .arg("-6")
        .arg("route")
        .arg("add")
        .arg(format!("{}/{}", format_ipv6(address), prefix_length))
        .arg("dev")
        .arg(interface)
        .arg("metric")
        .arg("100")
        .output();
}

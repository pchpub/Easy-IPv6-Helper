use std::process::Command;

use pnet::ipnetwork::IpNetwork;

use crate::mods::network::{
    interfaces::find_interface,
    ipv6::{format_ipv6, is_link_local_address, is_unique_local_address},
};

pub fn delete_all_public_ipv6_addr(interface_name: &str) -> Result<(), String> {
    match get_public_ipv6_addr(interface_name) {
        Ok(ipv6s) => {
            for (ipv6, prefix) in ipv6s {
                match del_ipv6_address(interface_name, &ipv6, prefix) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("删除IPv6地址失败: {}", e);
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("获取IPv6地址失败: {}", e)),
    }
}

pub fn get_public_ipv6_addr(interface_name: &str) -> Result<Vec<([u16; 8], u8)>, String> {
    let interface = find_interface(interface_name).ok_or(String::from("未找到以太网接口"))?;
    let ipv6s: Vec<([u16; 8], u8)> = interface
        .ips
        .iter()
        .filter(|ip| match ip {
            IpNetwork::V6(ipv6) => {
                !is_link_local_address(&ipv6.ip()) && !is_unique_local_address(&ipv6.ip())
            }
            _ => false,
        })
        .filter_map(|ip| match ip {
            IpNetwork::V6(ipv6) => Some((ipv6.ip().segments(), ipv6.prefix())),
            _ => None,
        })
        .collect::<Vec<_>>();
    Ok(ipv6s)
}

pub fn add_ipv6_address(
    interface: &str,
    address: &[u16; 8],
    prefix_length: u8,
) -> Result<(), String> {
    let output = Command::new("ip")
        .arg("-6")
        .arg("addr")
        .arg("add")
        .arg(format!("{}/{}", format_ipv6(address), prefix_length))
        .arg("dev")
        .arg(interface)
        .output();

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

    match output {
        Ok(output) if output.status.success() => {
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            Ok(())
        }
        Ok(output) => Err(format!(
            "Command failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )),
        Err(e) => Err(format!("Failed to execute process: {}", e)),
    }
}

pub fn del_ipv6_address(
    interface: &str,
    address: &[u16; 8],
    prefix_length: u8,
) -> Result<(), String> {
    let output = Command::new("ip")
        .arg("-6")
        .arg("addr")
        .arg("del")
        .arg(format!("{}/{}", format_ipv6(address), prefix_length))
        .arg("dev")
        .arg(interface)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            Ok(())
        }
        Ok(output) => Err(format!(
            "Command failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )),
        Err(e) => Err(format!("Failed to execute process: {}", e)),
    }
}

// pub fn del_error_route

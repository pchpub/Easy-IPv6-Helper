use pch_easy_ipv6::mods::config::CONFIG;
use pch_easy_ipv6::mods::handle::ra::handle_router_advertisement;
use pch_easy_ipv6::mods::network::interfaces::find_interface;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::icmpv6::{Icmpv6Packet, Icmpv6Types};
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // get interface name from command line arguments
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("config.json"));

    let config = match CONFIG.lock().unwrap().load(&config_path) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("加载配置文件失败: {}", e);
            return;
        }
    };

    let interface_name = config.upper_interface;

    // 创建并绑定数据包捕获通道
    loop {
        let interface = match find_interface(&interface_name) {
            Some(interface) => interface,
            None => {
                eprintln!("未找到以太网接口");
                sleep(Duration::from_secs(1));
                eprintln!("重新尝试寻找以太网接口");
                continue;
            }
        };

        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(_, rx)) => ((), rx),
            Ok(_) => {
                eprintln!("未找到以太网接口");
                sleep(Duration::from_secs(1));
                eprintln!("重新尝试创建数据包捕获通道");
                continue;
            }
            Err(e) => {
                eprintln!("创建数据包捕获通道失败: {}", e);
                sleep(Duration::from_secs(1));
                eprintln!("重新尝试创建数据包捕获通道");
                continue;
            }
        };

        println!("开始监听数据包");

        loop {
            match rx.next() {
                Ok(packet) => {
                    if let Some(ipv6_packet) = Ipv6Packet::new(packet) {
                        // 检查是否为ICMPv6包
                        if ipv6_packet.get_next_header()
                            == pnet::packet::ip::IpNextHeaderProtocols::Icmpv6
                        {
                            if let Some(icmpv6_packet) = Icmpv6Packet::new(ipv6_packet.payload()) {
                                if icmpv6_packet.get_icmpv6_type() == Icmpv6Types::RouterAdvert {
                                    handle_router_advertisement(&icmpv6_packet);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    if e.to_string().contains("os error 100") {
                        eprintln!("监听过程中出现错误: 接口不存在 ({})", e);
                        sleep(Duration::from_millis(10));
                    } else {
                        eprintln!("监听过程中出现错误: {}", e);
                        sleep(Duration::from_secs(1));
                    }
                    break;
                }
            }
        }
    }
}

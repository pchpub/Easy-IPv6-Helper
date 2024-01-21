use pnet::packet::icmpv6::ndp::NdpOptionTypes;
use pnet::packet::icmpv6::ndp::RouterAdvertPacket;
use pnet::packet::icmpv6::Icmpv6Packet;
use pnet::packet::Packet;

use crate::mods::config::CONFIG;
use crate::mods::host::config_interface::add_ipv6_address;
use crate::mods::host::config_interface::delete_all_public_ipv6_addr;
use crate::mods::host::config_interface::get_public_ipv6_addr;
use crate::mods::host::radvd::config_radvd;
use crate::mods::network::ipv6::format_ipv6;
use crate::mods::network::ipv6::generate_first_ipv6_with_prefix;
use crate::mods::network::ipv6::generate_unique_random_subnet;
use crate::mods::network::ipv6::ipv6_start_with_prefix;

pub fn handle_router_advertisement(icmpv6_packet: &Icmpv6Packet) {
    let router_ad = if let Some(router_ad) = RouterAdvertPacket::new(icmpv6_packet.packet()) {
        println!("New Router Advertisement packet received");
        println!("-------------------------------------------------");
        println!("Router Advertisement All: {:?}", router_ad);
        println!(
            "Router Advertisement Lifetime: {:?}",
            router_ad.get_lifetime()
        );
        println!("Router Advertisement Flags: {:?}", router_ad.get_flags());
        println!(
            "Router Advertisement ICMPv6 Code: {:?}",
            router_ad.get_icmpv6_code()
        );
        println!(
            "Router Advertisement ICMPv6 Type: {:?}",
            router_ad.get_icmpv6_type()
        );
        println!(
            "Router Advertisement Options: {:?}",
            router_ad.get_options_iter().collect::<Vec<_>>()
        );
        router_ad
    } else {
        println!("Not a Router Advertisement packet");
        return;
    };
    let ndp_options = router_ad
        .get_options_iter()
        .filter(|opt| opt.get_option_type() == NdpOptionTypes::PrefixInformation)
        .next();
    if let Some(ndp_option) = ndp_options {
        // print prefix information
        let data = ndp_option.payload();
        let prefix_info = parse_prefix_information(&data);
        prefix_info.print();
        let config = CONFIG.lock().unwrap();
        // println!("{:?}", config);

        {
            let ipv6s = match get_public_ipv6_addr(&config.next_interface) {
                Ok(ipv6s) => ipv6s,
                Err(e) => {
                    eprintln!("Failed to get ipv6 address : {}", e);
                    return;
                }
            };
            let mut need_update = true;
            for (ipv6, _) in ipv6s {
                need_update = false;
                if !ipv6_start_with_prefix(&ipv6, &prefix_info.prefix, prefix_info.prefix_length) {
                    need_update = true;
                    break;
                }
            }
            if !need_update {
                println!("The next interface's ipv6 address is already updated");
                return;
            } else {
                println!("The next interface's ipv6 address is not updated");
            }
        }
        println!("debug 1.1");
        // 生成2个随机的子网pd前缀 // fuck! 你妈的pd前缀怎么是64，电信你妈死了
        let subnet_prefix_length = config.subnet_prefix_length;
        // println!("debug 1.2");
        // let (subnet_prefix, subnet_prefix_length) = match generate_unique_random_subnet(
        //     &prefix_info.prefix,
        //     prefix_info.prefix_length,
        //     subnet_prefix_length,
        //     2,
        // ) {
        //     Ok(subnet_prefix) => subnet_prefix,
        //     Err(e) => {
        //         eprintln!("生成随机子网pd前缀失败: {}", e);
        //         return;
        //     }
        // };
        let (subnet_prefix, subnet_prefix_length) = {
            if prefix_info.prefix_length >= 64 {
                (prefix_info.prefix, 64)
            } else {
                match generate_unique_random_subnet(
                    &prefix_info.prefix,
                    prefix_info.prefix_length,
                    subnet_prefix_length,
                    1,
                ) {
                    Ok(subnet_prefix) => (subnet_prefix.0.get(0).unwrap().clone(), subnet_prefix.1),
                    Err(e) => {
                        eprintln!("生成随机子网pd前缀失败: {}", e);
                        return;
                    }
                }
            }
        };

        if config.features.config_interface {
            let next_interface_ipv6 =
                match generate_first_ipv6_with_prefix(&subnet_prefix, subnet_prefix_length) {
                    Ok(ipv6) => ipv6,
                    Err(e) => {
                        eprintln!("生成下一个接口的IPv6地址失败: {}", e);
                        return;
                    }
                };

            // 删除 next_interface 上其它公共ipv6地址
            match delete_all_public_ipv6_addr(config.next_interface.as_str()) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("删除下一个接口的其它公共IPv6地址失败: {}", e);
                    return;
                }
            };
            // 分配到 next_interface 上
            match add_ipv6_address(
                &config.next_interface,
                &next_interface_ipv6,
                subnet_prefix_length,
            ) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("附加IPv6地址失败: {}", e);
                    return;
                }
            }
        }
        if config.features.radvd {
            println!("debug 4");
            let radvd_prefix =
                match generate_first_ipv6_with_prefix(&subnet_prefix, subnet_prefix_length) {
                    Ok(ipv6) => ipv6,
                    Err(e) => {
                        eprintln!("生成Radvd前缀失败: {}", e);
                        return;
                    }
                };
            match config_radvd(
                &config.next_interface,
                &format_ipv6(&radvd_prefix),
                subnet_prefix_length,
            ) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("配置Radvd失败: {}", e);
                }
            };
        }
    } else {
        println!("Not a Prefix Information packet");
        return;
    }
}

pub struct PrefixInformation {
    pub prefix_length: u8,
    on_link_flag: bool,
    autonomous_address_configuration_flag: bool,
    pub valid_lifetime: u32,
    pub preferred_lifetime: u32,
    prefix: [u16; 8], // IPv6地址是128位，所以这里使用16字节的数组
}

impl PrefixInformation {
    fn print(&self) {
        println!("Prefix Length: {}", self.prefix_length);
        println!("On Link Flag: {}", self.on_link_flag);
        println!(
            "Autonomous Address Configuration Flag: {}",
            self.autonomous_address_configuration_flag
        );
        println!("Valid Lifetime: {}", self.valid_lifetime);
        println!("Preferred Lifetime: {}", self.preferred_lifetime);
        println!(
            "Prefix: {}",
            self.prefix
                .iter()
                .map(|x| format!("{:04x}", x))
                .collect::<Vec<_>>()
                .join(":")
        );
    }
}

fn parse_prefix_information(data: &[u8]) -> PrefixInformation {
    let prefix_length = data[0];
    let flags = data[1];
    let valid_lifetime = u32::from_be_bytes([data[2], data[3], data[4], data[5]]);
    let preferred_lifetime = u32::from_be_bytes([data[6], data[7], data[8], data[9]]);

    let mut prefix = [0u16; 8];
    for (i, chunk) in data[14..30].chunks(2).enumerate() {
        prefix[i] = u16::from_be_bytes([chunk[0], chunk[1]]);
    }

    let mut mask = [0u16; 8];
    for i in 0..8 {
        if prefix_length >= (i + 1) * 16 {
            mask[i as usize] = 0xffff;
        } else if prefix_length > i * 16 {
            mask[i as usize] = 0xffff << ((i + 1) * 16 - prefix_length);
        }
    }

    for i in 0..8 {
        prefix[i] = prefix[i] & mask[i]; // 强制修正前缀! 虽然没加没事，但是加上也没事？防止以后出问题 太棒了
    }

    PrefixInformation {
        prefix_length,
        on_link_flag: flags & 0x80 != 0,
        autonomous_address_configuration_flag: flags & 0x40 != 0,
        valid_lifetime,
        preferred_lifetime,
        prefix,
    }
}

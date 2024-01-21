#[cfg(feature = "unused_code")]
use rand::thread_rng;
use rand::Rng;
use std::net::Ipv6Addr;

#[cfg(feature = "unused_code")]
pub fn generate_random_ipv6_with_prefix(
    prefix: &[u16; 8],
    // prefix_length: u8,
    random_start_index: u8,
) -> Result<[u16; 8], String> {
    if random_start_index > 128 {
        return Err(String::from("Prefix length must be less than 128"));
    }
    let random_bytes: [u16; 8] = thread_rng().gen();

    let mut random_ipv6 = [0u16; 8];
    let mut mask = [0u16; 8];
    for i in 0..8 {
        if random_start_index >= (i + 1) * 16 {
            mask[i as usize] = 0xffff;
        } else if random_start_index > i * 16 {
            mask[i as usize] = 0xffff << ((i + 1) * 16 - random_start_index);
        }
    }
    for i in 0..8 {
        random_ipv6[i] = prefix[i] & mask[i] | random_bytes[i] & !mask[i];
    }
    Ok(random_ipv6)
}

#[cfg(feature = "unused_code")]
fn generate_random_ipv6() -> [u16; 8] {
    thread_rng().gen()
}

#[cfg(feature = "unused_code")]
fn generate_random_mac() -> [u8; 6] {
    thread_rng().gen()
}

pub fn generate_unique_random_subnet(
    prefix: &[u16; 8],
    main_prefix_length: u8,
    subnet_prefix_length: u8,
    number_of_subnets: usize,
) -> Result<(Vec<[u16; 8]>, u8), String> {
    let subnet_prefix_length = if subnet_prefix_length > 128 {
        subnet_prefix_length - 128 + main_prefix_length
    } else {
        subnet_prefix_length
    };
    if subnet_prefix_length > 128 {
        return Err(String::from("Subnet prefix length must be less than 128"));
    }
    if main_prefix_length > subnet_prefix_length {
        return Err(String::from("Mainnet prefix length must be less than 128"));
    } else if main_prefix_length == subnet_prefix_length {
        return Err(String::from(
            "Mainnet prefix length must be less than subnet prefix length",
        ));
    }

    let mut mainnet_mask = [0u16; 8];
    for i in 0..8 {
        if subnet_prefix_length >= (i + 1) * 16 {
            mainnet_mask[i as usize] = 0xffff;
        } else if main_prefix_length > i * 16 {
            mainnet_mask[i as usize] = 0xffff << (i * 16 + 16 - main_prefix_length);
        }
    }

    let mut subnet_mask: [u16; 8] = [0u16; 8];
    for i in 0..8 {
        if subnet_prefix_length >= (i + 1) * 16 {
            subnet_mask[i as usize] = 0xffff;
        } else if subnet_prefix_length > i * 16 {
            subnet_mask[i as usize] = 0xffff << (i * 16 + 16 - subnet_prefix_length);
        }
    }

    for i in 0..8 {
        subnet_mask[i] = (!mainnet_mask[i]) & subnet_mask[i];
    }

    let mut subnet_prefixs: Vec<[u16; 8]> = Vec::with_capacity(number_of_subnets);
    let mut rng = rand::thread_rng();
    loop {
        if subnet_prefixs.len() >= number_of_subnets {
            break;
        }
        let mut subnet_prefix: [u16; 8] = rng.gen();
        for i in 0..8 {
            subnet_prefix[i] = (subnet_prefix[i] & subnet_mask[i]) | prefix[i];
        }
        if !subnet_prefixs.contains(&subnet_prefix) {
            subnet_prefixs.push(subnet_prefix);
        }
    }

    Ok((subnet_prefixs, subnet_prefix_length))
}

pub fn generate_first_ipv6_with_prefix(
    prefix: &[u16; 8],
    prefix_length: u8,
) -> Result<[u16; 8], String> {
    if prefix_length > 128 {
        return Err(String::from("Prefix length must be less than 128"));
    }
    let first_bytes: [u16; 8] = [0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 1u16];

    let mut first_ipv6 = [0u16; 8];
    let mut mask = [0u16; 8];
    for i in 0..8 {
        if prefix_length >= (i + 1) * 16 {
            mask[i as usize] = 0xffff;
        } else if prefix_length > i * 16 {
            mask[i as usize] = 0xffff << ((i + 1) * 16 - prefix_length);
        }
    }
    for i in 0..8 {
        first_ipv6[i] = (prefix[i] & mask[i]) + first_bytes[i];
    }
    Ok(first_ipv6)
}

pub fn is_unique_local_address(addr: &Ipv6Addr) -> bool {
    let first_byte = addr.segments()[0];
    (first_byte & 0xfe00) == 0xfc00
}

pub fn is_link_local_address(addr: &Ipv6Addr) -> bool {
    let first_byte = addr.segments()[0];
    (first_byte & 0xffc0) == 0xfe80
}

pub fn format_ipv6(addr: &[u16; 8]) -> String {
    let ipv6_addr = Ipv6Addr::new(
        addr[0], addr[1], addr[2], addr[3], addr[4], addr[5], addr[6], addr[7],
    );

    ipv6_addr.to_string()
}

pub fn ipv6_start_with_prefix(addr: &[u16; 8], prefix: &[u16; 8], prefix_length: u8) -> bool {
    let mut mask = [0u16; 8];
    for i in 0..8 {
        if prefix_length >= (i + 1) * 16 {
            mask[i as usize] = 0xffff;
        } else if prefix_length > i * 16 {
            mask[i as usize] = 0xffff << ((i + 1) * 16 - prefix_length);
        }
    }
    for i in 0..8 {
        if addr[i] & mask[i] != prefix[i] & mask[i] {
            return false;
        }
    }
    true
}

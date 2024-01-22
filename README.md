# Easy-IPv6-Helper

## Description

A helper to manage ipv6-pd.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

1. **Clone the Repository**

   ```sh
   git clone https://github.com/pchpub/Easy-IPv6-Helper.git
   ```

2. **Compile the Code**

   ```sh
   cd Easy-IPv6-Helper
   sh build.sh
   chmod +x ./easy-ipv6-helper
   ```

3. **Configure**

   ```sh
   mv config.example.json config.json
   vim config.json
   ```

4. **Run it**

   ```sh
    ./easy-ipv6-helper config.json
   ```

## Usage

   edit config.json with comments

   ```json
{
    "subnet_prefix_length": 130, 
    // subnet prefix length
    // if you want to get a /64 subnet, set it to 64
    // if you want to set it to the pd prefix length + 2, set it to 130
    "upper_interface": "ppp0",
    // the interface that get the pd
    "next_interface": "br-lan",
    // the interface that you want to configure
    "features": {
        "radvd": true,
        // auto config radvd
        "dhcpv6": true,
        // auto config dhcpv6 (not support yet)
        "ndproxy": true,
        // auto config ndproxy(not support yet)
        "config_interface": true,
        // auto config interface
        "router": true
        // auto config router
    }
}
   
   ```

## Contributing

Any contributions you make are **greatly appreciated**.

## License

This project is licensed under the GNU General Public License v3.0 license. see the [LICENSE](LICENSE) file for details.

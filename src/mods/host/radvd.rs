// 自动配置radvd

use std::{fs::File, io::Write};

pub fn config_radvd(interface_name: &str, prefix: &str, prefix_length: u8) -> Result<(), String> {
    let mut config = String::from("interface ");
    config.push_str(interface_name);
    config.push_str(
        r#"
{
    MinRtrAdvInterval 3;
    MaxRtrAdvInterval 4;
    AdvSendAdvert on;
    AdvManagedFlag on;
    prefix "#,
    );
    config.push_str(prefix);
    config.push('/');
    config.push_str(prefix_length.to_string().as_str());
    config.push_str(
        r#" 
    {
        AdvValidLifetime 14300; 
        AdvPreferredLifetime 14200; 
    };
};"#,
    );
    let config_file = "/etc/radvd.conf";
    let mut file = File::create(config_file).map_err(|e| e.to_string())?;
    file.write_all(config.as_bytes())
        .map_err(|e| e.to_string())?;
    // systemctl restart radvd
    let output = std::process::Command::new("systemctl")
        .arg("restart")
        .arg("radvd")
        .output()
        .map_err(|e| e.to_string())?;
    match output.status.code() {
        Some(0) => Ok(()),
        Some(code) => Err(format!("systemctl restart radvd failed: {}", code)),
        None => Err(String::from("systemctl restart radvd failed")),
    }
}

use std::net::UdpSocket;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Probe {
    #[serde(rename = "Uuid")]
    uuid: String,
    #[serde(rename = "Types")]
    types: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ProbeMatch {
    #[serde(rename = "Uuid")]
    uuid: String,
    #[serde(rename = "Types")]
    types: String,
    #[serde(rename = "DeviceType")]
    device_type: String,
    #[serde(rename = "DeviceDescription")]
    device_description: String,
    #[serde(rename = "DeviceSN")]
    device_sn: String,
    #[serde(rename = "CommandPort")]
    command_port: u32,
    #[serde(rename = "HttpPort")]
    http_port: String,
    #[serde(rename = "MAC")]
    mac: String,
    #[serde(rename = "IPv4Address")]
    ipv4_address: String,
    #[serde(rename = "IPv4SubnetMask")]
    ipv4_subnet_mask: String,
    #[serde(rename = "IPv4Gateway")]
    ipv4_gateway: String,
    #[serde(rename = "IPv6Address")]
    ipv6_address: String,
    #[serde(rename = "IPv6Gateway")]
    ipv6_gateway: String,
    #[serde(rename = "IPv6MaskLen")]
    ipv6_mask_len: u32,
    #[serde(rename = "DHCP")]
    dhcp: String,
    #[serde(rename = "AnalogChannelNum")]
    analog_channel_num: u32,
    #[serde(rename = "DigitalChannelNum")]
    digital_channel_num: u32,
    #[serde(rename = "SoftwareVersion")]
    software_version: String,
    #[serde(rename = "DSPVersion")]
    dsp_version: String,
    #[serde(rename = "BootTime")]
    boot_time: String,
    #[serde(rename = "ResetAbility")]
    reset_ability: bool,
    #[serde(rename = "DiskNumber")]
    disk_number: u32,
    #[serde(rename = "Activated")]
    activated: bool,
    #[serde(rename = "PasswordResetAbility")]
    password_reset_ability: bool,
    #[serde(rename = "PasswordResetModeSecond")]
    password_reset_mode_second: bool,
    #[serde(rename = "SupportHCPlatform")]
    support_hc_platform: bool,
    #[serde(rename = "HCPlatformEnable")]
    // "true" or "flase", good job hikvision
    hc_platform_enable: String,
    #[serde(rename = "IsModifyVerificationCode")]
    is_modify_verification_code: String,
}



fn main() {
    let socket:UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_broadcast(true).unwrap();
    socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

    let probe = Probe {
        types: "inquiry".to_string(),
        uuid: Uuid::new_v4().to_string(),
    };

    println!("Starting discovery!");
    socket.send_to(serde_xml_rs::to_string(&probe).unwrap().as_bytes(), "255.255.255.255:37020").unwrap();
    let mut recv_buff = [0u8; 5000];

    println!("Desc\t\tip");
    while let Ok((n, _addr)) = socket.recv_from(&mut recv_buff) {
        let buf = &recv_buff[0..n];

        let data = std::str::from_utf8(buf).unwrap();
        let parsed: ProbeMatch = serde_xml_rs::from_str(data).unwrap();

        println!("{}\t{}", parsed.device_description, parsed.ipv4_address);
    }
}

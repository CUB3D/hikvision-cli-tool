use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::time::Duration;
use uuid::Uuid;

//Questions to answer:
// What is the method for calculating the password
// How does activation work
// How does the password reset work

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "Types")]
enum Probe {
    #[serde(rename = "Update")]
    Update {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "PWErrorParse")]
        pw_error_parse: String,
        #[serde(rename = "MAC")]
        mac: String,
        //TODO: how is this derived, base64(something(password))
        #[serde(rename = "Password")]
        password: String,
        #[serde(rename = "IPv4Address")]
        ipv4_address: String,
        #[serde(rename = "CommandPort")]
        command_port: String,
        #[serde(rename = "HttpPort")]
        http_port: String,
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
        dhcp: bool,
        #[serde(rename = "SDKOverTLSPort")]
        sdk_over_tls_port: u32,
    },
    #[serde(rename = "inquiry")]
    Inquiry {
        #[serde(rename = "Uuid")]
        uuid: String,
    },
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
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_broadcast(true).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    let probe = Probe::Inquiry {
        uuid: Uuid::new_v4().to_string(),
    };

    // let probe = Probe::Update {
    //     uuid: "".to_string(),
    //     pw_error_parse: "".to_string(),
    //     mac: "".to_string(),
    //     password: "".to_string(),
    //     ipv4_address: "".to_string(),
    //     command_port: "".to_string(),
    //     http_port: "".to_string(),
    //     ipv4_subnet_mask: "".to_string(),
    //     ipv4_gateway: "".to_string(),
    //     ipv6_address: "".to_string(),
    //     ipv6_gateway: "".to_string(),
    //     ipv6_mask_len: 0,
    //     dhcp: false,
    //     sdk_over_tls_port: 0
    // };

    println!("Starting discovery!");
    socket
        .send_to(
            serde_xml_rs::to_string(&probe).unwrap().as_bytes(),
            "255.255.255.255:37020",
        )
        .unwrap();
    let mut recv_buff = [0u8; 5000];

    println!("Desc\t\tip");
    while let Ok((n, _addr)) = socket.recv_from(&mut recv_buff) {
        let buf = &recv_buff[0..n];

        let data = std::str::from_utf8(buf).unwrap();
        let parsed: ProbeMatch = serde_xml_rs::from_str(data).unwrap();

        println!("{}\t{}", parsed.device_description, parsed.ipv4_address);
    }
}

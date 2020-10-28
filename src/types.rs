use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Password(String);

impl Password {
    pub fn hash(password: &str) -> Self {
        Self(base64::encode(md5::compute(password).0))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "Types")]
pub enum Probe {
    #[serde(rename = "update")]
    Update {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "PWErrorParse")]
        pw_error_parse: String,
        #[serde(rename = "MAC")]
        mac: String,
        /// base64(md5(password))
        /// In some cases this is also base64(md5(sha256(username+something+password)))
        /// In some other cases this is encrypted with (AES?) as (base64(encrypt(one_of_the_above_hashes...))?)
        #[serde(rename = "Password")]
        password: Password,
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
pub struct ProbeMatchSuccessBody {
    #[serde(rename = "DeviceType")]
    pub device_type: String,
    #[serde(rename = "DeviceDescription")]
    pub device_description: String,
    #[serde(rename = "DeviceSN")]
    pub device_sn: String,
    #[serde(rename = "CommandPort")]
    pub command_port: u32,
    #[serde(rename = "HttpPort")]
    pub http_port: String,
    #[serde(rename = "MAC")]
    pub mac: String,
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: String,
    #[serde(rename = "IPv4SubnetMask")]
    pub ipv4_subnet_mask: String,
    #[serde(rename = "IPv4Gateway")]
    pub ipv4_gateway: String,
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: String,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename = "IPv6MaskLen")]
    pub ipv6_mask_len: u32,
    #[serde(rename = "DHCP")]
    pub dhcp: bool,
    #[serde(rename = "AnalogChannelNum")]
    pub analog_channel_num: u32,
    #[serde(rename = "DigitalChannelNum")]
    pub digital_channel_num: u32,
    #[serde(rename = "SoftwareVersion")]
    pub software_version: String,
    #[serde(rename = "DSPVersion")]
    pub dsp_version: String,
    #[serde(rename = "BootTime")]
    pub boot_time: String,
    #[serde(rename = "ResetAbility")]
    pub reset_ability: bool,
    #[serde(rename = "DiskNumber")]
    pub disk_number: u32,
    #[serde(rename = "Activated")]
    pub activated: bool,
    #[serde(rename = "PasswordResetAbility")]
    pub password_reset_ability: bool,
    #[serde(rename = "PasswordResetModeSecond")]
    pub password_reset_mode_second: bool,
    #[serde(rename = "SupportHCPlatform")]
    pub support_hc_platform: bool,
    #[serde(rename = "HCPlatformEnable")]
    // "true" or "flase", good job hikvision
    pub hc_platform_enable: String,
    #[serde(rename = "IsModifyVerificationCode")]
    pub is_modify_verification_code: String,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ProbeMatchResult {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "success")]
    Success,
}

//todo: error (<Result>String</><
#[derive(Deserialize, Debug, Clone)]
pub struct ProbeMatch {
    #[serde(rename = "Uuid")]
    pub uuid: String,
    #[serde(rename = "Types")]
    pub types: String,

    #[serde(rename = "Result")]
    result: Option<ProbeMatchResult>,
    #[serde(rename = "DeviceType")]
    device_type: Option<String>,
    #[serde(rename = "DeviceDescription")]
    device_description: Option<String>,
    #[serde(rename = "DeviceSN")]
    device_sn: Option<String>,
    #[serde(rename = "CommandPort")]
    command_port: Option<u32>,
    #[serde(rename = "HttpPort")]
    http_port: Option<String>,
    #[serde(rename = "MAC")]
    mac: Option<String>,
    #[serde(rename = "IPv4Address")]
    ipv4_address: Option<String>,
    #[serde(rename = "IPv4SubnetMask")]
    ipv4_subnet_mask: Option<String>,
    #[serde(rename = "IPv4Gateway")]
    ipv4_gateway: Option<String>,
    #[serde(rename = "IPv6Address")]
    ipv6_address: Option<String>,
    #[serde(rename = "IPv6Gateway")]
    ipv6_gateway: Option<String>,
    #[serde(rename = "IPv6MaskLen")]
    ipv6_mask_len: Option<u32>,
    #[serde(rename = "DHCP")]
    dhcp: Option<bool>,
    #[serde(rename = "AnalogChannelNum")]
    analog_channel_num: Option<u32>,
    #[serde(rename = "DigitalChannelNum")]
    digital_channel_num: Option<u32>,
    #[serde(rename = "SoftwareVersion")]
    software_version: Option<String>,
    #[serde(rename = "DSPVersion")]
    dsp_version: Option<String>,
    #[serde(rename = "BootTime")]
    boot_time: Option<String>,
    #[serde(rename = "ResetAbility")]
    reset_ability: Option<bool>,
    #[serde(rename = "DiskNumber")]
    disk_number: Option<u32>,
    #[serde(rename = "Activated")]
    activated: Option<bool>,
    #[serde(rename = "PasswordResetAbility")]
    password_reset_ability: Option<bool>,
    #[serde(rename = "PasswordResetModeSecond")]
    password_reset_mode_second: Option<bool>,
    #[serde(rename = "SupportHCPlatform")]
    support_hc_platform: Option<bool>,
    #[serde(rename = "HCPlatformEnable")]
    // "true" or "flase", good job hikvision
    hc_platform_enable: Option<String>,
    #[serde(rename = "IsModifyVerificationCode")]
    is_modify_verification_code: Option<String>,
}

impl ProbeMatch {
    pub fn payload(self) -> Result<ProbeMatchSuccessBody, String> {
        if let Some(ProbeMatchResult::Failed) = self.result {
            Err("Camera indicated failure".to_string())
        } else {
            Ok(ProbeMatchSuccessBody {
                device_type: self.device_type.unwrap(),
                device_description: self.device_description.unwrap(),
                device_sn: self.device_sn.unwrap(),
                command_port: self.command_port.unwrap(),
                http_port: self.http_port.unwrap(),
                mac: self.mac.unwrap(),
                ipv4_address: self.ipv4_address.unwrap(),
                ipv4_subnet_mask: self.ipv4_subnet_mask.unwrap(),
                ipv4_gateway: self.ipv4_gateway.unwrap(),
                ipv6_address: self.ipv6_address.unwrap(),
                ipv6_gateway: self.ipv6_gateway.unwrap(),
                ipv6_mask_len: self.ipv6_mask_len.unwrap(),
                dhcp: self.dhcp.unwrap(),
                analog_channel_num: self.analog_channel_num.unwrap(),
                digital_channel_num: self.digital_channel_num.unwrap(),
                software_version: self.software_version.unwrap(),
                dsp_version: self.dsp_version.unwrap(),
                boot_time: self.boot_time.unwrap(),
                reset_ability: self.reset_ability.unwrap(),
                disk_number: self.disk_number.unwrap(),
                activated: self.activated.unwrap(),
                password_reset_ability: self.password_reset_ability.unwrap(),
                password_reset_mode_second: self.password_reset_mode_second.unwrap(),
                support_hc_platform: self.support_hc_platform.unwrap(),
                hc_platform_enable: self.hc_platform_enable.unwrap(),
                is_modify_verification_code: self.is_modify_verification_code.unwrap(),
            })
        }
    }
}

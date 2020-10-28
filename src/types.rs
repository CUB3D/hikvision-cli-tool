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
pub struct ProbeMatch {
    #[serde(rename = "Uuid")]
    pub uuid: String,
    #[serde(rename = "Types")]
    pub types: String,
    #[serde(rename = "DeviceType")]
    pub device_type: String,
    #[serde(rename = "DeviceDescription")]
    pub(crate) device_description: String,
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

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Password(String);

impl Password {
    pub fn hash(password: &str) -> Self {
        Self(base64::encode(md5::compute(password).0))
    }
}

impl From<Password> for String {
    fn from(p: Password) -> Self {
        p.0
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
        http_port: u32,
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
    MailReset {},
    #[serde(rename = "reset")]
    Reset {},
    #[serde(rename = "GetQRcodes")]
    GetQRCodes {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
    },
    #[serde(rename = "getcode")]
    GetCode {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
    },
    #[serde(rename = "exchangecode")]
    ExchangeCode {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
        #[serde(rename = "Code")]
        code: String,
    },
    #[serde(rename = "activate")]
    Activate {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
        ///TODO: is this a different format to the normal password, it seems longer (maybe sha256 format from above)
        #[serde(rename = "Password")]
        password: String,
    },
    #[serde(rename = "getencryptstring")]
    GetEncryptString {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
    },
    #[serde(rename = "deviceTypeUnlockCode")]
    DeviceTypeUnlockCode {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
    },
    #[serde(rename = "deviceTypeCustom")]
    DeviceTypeCustom {},
    ExportGUID {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
        #[serde(rename = "Password")]
        password: Password
    },
    #[serde(rename = "getsecurityquestion")]
    GetSecurityQuestion {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
    },
    #[serde(rename = "setsecurityquestion")]
    SetSecurityQuestions,
    GUIDReset,
    AnswerReset,
    SetMailBox,
    SetHCPlatform,
    SetVerificationCode,
    #[serde(rename = "getBindList")]
    GetBindList {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
    },
    #[serde(rename = "setBindList")]
    SetBindList,
    #[serde(rename = "restoreInactive")]
    RestoreInactive,
    #[serde(rename = "setWifiRegion")]
    SetWifiRegion,
    #[serde(rename = "lamp")]
    Lamp,
    #[serde(rename = "selfCheck")]
    SelfCheck {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
    },
    #[serde(rename = "diskLocate")]
    DiskLocate {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "MAC")]
        mac: String,
    },
    #[serde(rename = "setChannelDefaultPassword")]
    SetChannelDefaultPassword,
    #[serde(rename = "wifiParamCfg")]
    WifiParamConfig,
    #[serde(rename = "wifiParamCheck")]
    WifiParamCheck,
    EHomeEnable,
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
    pub http_port: u32,
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
    pub hc_platform_enable: HvTypoBool,
    #[serde(rename = "IsModifyVerificationCode")]
    pub is_modify_verification_code: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum HvTypoBool {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "flase")]
    Flase,
}

impl From<&HvTypoBool> for bool {
    fn from(b: &HvTypoBool) -> Self {
        match *b {
            HvTypoBool::True => true,
            HvTypoBool::Flase => false,
        }
    }
}

impl std::fmt::Display for HvTypoBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b: bool = self.into();
        b.fmt(f)
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ProbeMatchResult {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "success")]
    Success,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "Types")]
pub struct ProbeMatchBody {

}

#[derive(Deserialize, Debug, Clone)]
pub enum ProbeMatch2 {
    #[serde(rename = "inquire")]
    Inquire {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "Result")]
        result: ProbeMatchResult,
    },
    #[serde(rename = "getcode")]
    GetCode {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "Result")]
        result: ProbeMatchResult,
        #[serde(rename = "Code")]
        code: String
    },
    #[serde(rename = "getencryptstring")]
    GetEncryptString {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "Result")]
        result: ProbeMatchResult,
        #[serde(rename = "EncryptString")]
        encrypt_string: String
    },
    #[serde(rename = "exchangecode")]
    ExchangeCode {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "Result")]
        result: ProbeMatchResult,
        #[serde(rename = "Code")]
        code: String
    },
    // Yes the space is intentional...
    #[serde(rename = "activate ")]
    Activate {
        #[serde(rename = "Uuid")]
        uuid: String,
        #[serde(rename = "Result")]
        result: ProbeMatchResult,
    }
}

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
    http_port: Option<u32>,
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
    hc_platform_enable: Option<HvTypoBool>,
    #[serde(rename = "IsModifyVerificationCode")]
    is_modify_verification_code: Option<String>,
}

impl ProbeMatch {
    fn into_success_body(self) -> Option<ProbeMatchSuccessBody> {
        Some(ProbeMatchSuccessBody {
            device_type: self.device_type?,
            device_description: self.device_description?,
            device_sn: self.device_sn?,
            command_port: self.command_port?,
            http_port: self.http_port?,
            mac: self.mac?,
            ipv4_address: self.ipv4_address?,
            ipv4_subnet_mask: self.ipv4_subnet_mask?,
            ipv4_gateway: self.ipv4_gateway?,
            ipv6_address: self.ipv6_address?,
            ipv6_gateway: self.ipv6_gateway?,
            ipv6_mask_len: self.ipv6_mask_len?,
            dhcp: self.dhcp?,
            analog_channel_num: self.analog_channel_num?,
            digital_channel_num: self.digital_channel_num?,
            software_version: self.software_version?,
            dsp_version: self.dsp_version?,
            boot_time: self.boot_time?,
            reset_ability: self.reset_ability?,
            disk_number: self.disk_number?,
            activated: self.activated?,
            password_reset_ability: self.password_reset_ability?,
            password_reset_mode_second: self.password_reset_mode_second?,
            support_hc_platform: self.support_hc_platform?,
            hc_platform_enable: self.hc_platform_enable?,
            is_modify_verification_code: self.is_modify_verification_code?,
        })
    }

    pub fn payload(self) -> Result<ProbeMatchSuccessBody, String> {
        if let Some(ProbeMatchResult::Failed) = self.result {
            Err("Camera indicated failure".to_string())
        } else {
            match self.into_success_body() {
                Some(x) => Ok(x),
                None => Err("Invalid response".to_string()),
            }
        }
    }
}

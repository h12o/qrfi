use clap::ValueEnum;

/// Represents a Wi-Fi SSID.
///
/// Validation ensures that the length is between 1 and 32 bytes.
///
/// # Example
///
/// ```
/// use qrfi::Ssid;
///
/// let valid_ssid = Ssid("SSID".to_string());
/// assert!(valid_ssid.validate().is_ok());
///
/// let empty_ssid = Ssid("".to_string());
/// assert!(empty_ssid.validate().is_err());
/// ```
pub struct Ssid(pub String);
impl Ssid {
    /// Validates the SSID by checking its length.
    pub fn validate(&self) -> Result<(), String> {
        match self.0.len() {
            0 => Err("SSID cannot be empty.".to_string()),
            1..=32 => Ok(()),
            n => Err(format!(
                "SSID is too long ({} bytes). It must be between 1 and 32 bytes.", n
            )),
        }
    }
    /// Escapes special characters in the SSID for the MECARD-like syntax.
    pub fn escape(&self) -> String {
        mecardify(&self.0)
    }
}

/// Represents a Wi-Fi password and its authentication method.
///
/// # Example
///
/// ```
/// use qrfi::{Password, AuthType};
///
/// let pass = Password {
///     value: Some("PASSWORD".to_string()),
///     auth_type: AuthType::Wpa,
/// };
/// assert!(pass.validate().is_ok());
/// ```
pub struct Password {
    /// The password value, which can be `None` for open networks (`nopass`).
    pub value: Option<String>,
    /// The authentication type associated with the password.
    pub auth_type: AuthType,
}
impl Password {
    /// Validates the password based on the authentication type.
    pub fn validate(&self) -> Result<(), String> {
        let p = self.value.as_deref().unwrap_or("");
        let len = p.len();
        let is_hex = !p.is_empty() && p.chars().all(|c| c.is_ascii_hexdigit());
        let is_printable_ascii = !p.is_empty() && p.is_ascii() && p.chars().all(|c| (0x20..=0x7E).contains(&(c as u8)));
        let unit = if len == 1 { "byte" } else { "bytes" };
        let kind = if is_hex { "hex" } else { "string" };
        let current_info = format!("current: {} {} {}", len, unit, kind);
        match self.auth_type {
            AuthType::Nopass => {
                if !p.is_empty() {
                    return Err("Password should not be provided for 'nopass'.".to_string());
                }
            }
            AuthType::Wpa => {
                let is_valid_hex = len == 64 && is_hex;
                let is_valid_ascii = (8..=63).contains(&len) && is_printable_ascii;
                let char_type = if is_printable_ascii { "ASCII" } else { "non-ASCII" };
                if !(is_valid_ascii || is_valid_hex) {
                    return Err(format!(
                        "WPA passphrase must be 8-63 printable ASCII characters, or 64 hex digits ({}, {}).",
                        current_info, char_type
                    ));
                }
            }
            AuthType::Wep => {
                let is_valid_hex = (len == 10 || len == 26) && is_hex;
                if !([5, 13].contains(&len) || is_valid_hex) {
                    return Err(format!(
                        "WEP password must be 5 or 13 characters, or 10 or 26 hex digits ({}).",
                        current_info
                    ));
                }
            }
        }
        Ok(())
    }
    /// Escapes special characters in the password for the MECARD-like syntax.
    pub fn escape(&self) -> String {
        mecardify(self.value.as_deref().unwrap_or_default())
    }
}

/// Represents a Wi-Fi configuration and handles its conversion to the MECARD-like syntax proposed by ZXing.
///
/// # Example
///
/// ```
/// use qrfi::{Wifi, Ssid, Password, AuthType};
///
/// let wifi = Wifi {
///     ssid: Ssid("SSID".to_string()),
///     password: Password {
///         value: Some("PASSWORD".to_string()),
///         auth_type: AuthType::Wpa,
///     },
///     hidden: false,
/// };
///
/// assert_eq!(wifi.to_mecard(), "WIFI:S:SSID;T:WPA;P:PASSWORD;H:false;;");
/// ```
pub struct Wifi {
    /// The SSID (Service Set Identifier) of the Wi-Fi network.
    pub ssid: Ssid,
    /// The password and its associated authentication method.
    pub password: Password,
    /// Whether the Wi-Fi network's SSID is hidden (not broadcasted).
    pub hidden: bool,
}
impl Wifi {
    /// Validates the Wi-Fi configuration by checking both the SSID and password.
    pub fn validate(&self) -> Result<(), String> {
        self.ssid.validate()?;
        self.password.validate()?;
        Ok(())
    }
    /// Converts the Wi-Fi configuration into the MECARD-like syntax.
    pub fn to_mecard(&self) -> String {
        format!(
            "WIFI:S:{};T:{};P:{};H:{};;",
            self.ssid.escape(),
            self.password.auth_type,
            self.password.escape(),
            if self.hidden { "true" } else { "false" }
        )
    }
}

/// Escapes special characters for the MECARD-like syntax.
///
/// The four characters `:`, `;`, `,`, and `\` are escaped with a backslash.
///
/// # Example
///
/// ```
/// use qrfi::mecardify;
/// 
/// assert_eq!(mecardify("Example:SSID"), "Example\\:SSID");
/// assert_eq!(mecardify("A;B,C\\D"), "A\\;B\\,C\\\\D");
/// ```
pub fn mecardify(s: &str) -> String {
    let mut mecardified = String::new();
    for c in s.chars() {
        if matches!(c, ',' | ':' | ';' | '\\' ) {
            mecardified.push('\\');
        }
        mecardified.push(c);
    }
    mecardified
}

/// Supported Wi-Fi authentication types.
///
/// This enum corresponds to the `T:` (Authentication Type) field in the Wi-Fi network configuration syntax.
///
/// # Example
///
/// ```
/// use qrfi::AuthType;
///
/// let default_auth = AuthType::default();
/// assert_eq!(default_auth, AuthType::Wpa);
/// assert_eq!(format!("{}", default_auth), "WPA");
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum AuthType {
    /// WEP (Wired Equivalent Privacy).
    #[value(name = "WEP")]
    Wep,
    /// WPA, WPA2, or WPA3 (Wi-Fi Protected Access).
    #[default]
    #[value(name = "WPA")]
    Wpa,
    /// No password required (Open network).
    #[value(name = "nopass")]
    Nopass,
}
impl std::fmt::Display for AuthType {
    /// Formats the authentication type for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuthType::Wep => write!(f, "WEP"),
            AuthType::Wpa => write!(f, "WPA"),
            AuthType::Nopass => write!(f, "nopass"),
        }
    }
}

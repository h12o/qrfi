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
/// let valid_ssid = Ssid::new("SSID".to_string());
/// assert!(valid_ssid.is_ok());
///
/// let empty_ssid = Ssid::new("".to_string());
/// assert!(empty_ssid.is_err());
/// ```
pub struct Ssid(String);
impl Ssid {
    /// Constructor that validates the SSID.
    pub fn new(s: String) -> Result<Self, String> {
        let ssid = Self(s);
        ssid.validate()?;
        Ok(ssid)
    }
    /// Internal validation logic.
    fn validate(&self) -> Result<(), String> {
        match self.0.len() {
            0 => Err("SSID cannot be empty.".to_string()),
            1..=32 => Ok(()),
            n => Err(format!(
                "SSID is too long ({} bytes). It must be between 1 and 32 bytes.", n
            )),
        }
    }
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
/// let pass = Password::new(Some("PASSWORD".to_string()), AuthType::Wpa);
/// assert!(pass.is_ok());
/// ```
pub struct Password {
    /// The password value, which can be `None` for open networks (`nopass`).
    value: Option<String>,
    /// The authentication type associated with the password.
    auth_type: AuthType,
}
impl Password {
    /// Constructor that enforces business rules:
    /// If AuthType is Nopass, the password value is forced to None.
    pub fn new(value: Option<String>, auth_type: AuthType) -> Result<Self, String> {
        let actual_value = if auth_type == AuthType::Nopass {
            None
        } else {
            value
        };

        let pass = Self {
            value: actual_value,
            auth_type,
        };
        pass.validate()?;
        Ok(pass)
    }

    fn validate(&self) -> Result<(), String> {
        let p = self.value.as_deref().unwrap_or("");
        let len = p.len();
        let is_hex = !p.is_empty() && p.chars().all(|c| c.is_ascii_hexdigit());
        let is_printable_ascii = !p.is_empty() && p.is_ascii() && p.chars().all(|c| (0x20..=0x7E).contains(&(c as u8)));

        match self.auth_type {
            AuthType::Nopass => {
                if !p.is_empty() {
                    return Err("Password should not be provided for 'nopass'.".to_string());
                }
            }
            AuthType::Wpa => {
                let is_valid_hex = len == 64 && is_hex;
                let is_valid_ascii = (8..=63).contains(&len) && is_printable_ascii;
                if !(is_valid_ascii || is_valid_hex) {
                    return Err("WPA passphrase must be 8-63 printable ASCII characters, or 64 hex digits.".to_string());
                }
            }
            AuthType::Wep => {
                let is_valid_hex = (len == 10 || len == 26) && is_hex;
                if !([5, 13].contains(&len) || is_valid_hex) {
                    return Err("WEP password must be 5 or 13 characters, or 10 or 26 hex digits.".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn escape(&self) -> String {
        mecardify(self.value.as_deref().unwrap_or_default())
    }

    pub fn auth_type(&self) -> AuthType {
        self.auth_type
    }
}

/// Represents a Wi-Fi configuration and handles its conversion to the MECARD-like syntax proposed by ZXing.
///
/// # Example
///
/// ```
/// use qrfi::{Wifi, Ssid, Password, AuthType};
///
/// let ssid = Ssid::new("SSID".to_string()).unwrap();
/// let password = Password::new(Some("PASSWORD".to_string()), AuthType::Wpa).unwrap();
/// let wifi = Wifi::new(ssid, password, false);
///
/// assert_eq!(wifi.to_mecard(), "WIFI:S:SSID;T:WPA;P:PASSWORD;H:false;;");
/// ```
pub struct Wifi {
    /// The SSID (Service Set Identifier) of the Wi-Fi network.
    ssid: Ssid,
    /// The password and its associated authentication method.
    password: Password,
    /// Whether the Wi-Fi network's SSID is hidden (not broadcasted).
    hidden: bool,
}
impl Wifi {
    /// Since Ssid and Password are already validated, Wifi::new is always safe.
    pub fn new(ssid: Ssid, password: Password, hidden: bool) -> Self {
        Self { ssid, password, hidden }
    }

    pub fn to_mecard(&self) -> String {
        format!(
            "WIFI:S:{};T:{};P:{};H:{};;",
            self.ssid.escape(),
            self.password.auth_type(),
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

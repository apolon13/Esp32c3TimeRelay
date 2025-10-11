use std::fmt::{Display, Formatter};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};

pub struct Credentials {
    ssid: String,
    password: String,
    auth_method: AuthMethod,
}

impl Display for Credentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("ssid", &self.ssid)
            .field("password", &self.password)
            .field("auth_method", &self.auth_method)
            .finish()
    }
}

impl Credentials {
    pub fn new(ssid: String, password: String) -> Credentials {
        Credentials {
            auth_method: match password.is_empty() { 
                true => AuthMethod::None,
                false => AuthMethod::WPA2Personal
            },
            ssid,
            password,
        }
    }
}

pub struct Connection {
    credentials: Credentials,
    modem: Modem,
    sys_loop: EspSystemEventLoop,
}

impl Connection {
    pub fn new(credentials: Credentials, modem: Modem, sys_loop: EspSystemEventLoop) -> Self {
        Connection {
            credentials,
            modem,
            sys_loop,
        }
    }

    pub fn init(self) -> Result<EspWifi<'static>, EspError> {
        let mut esp_wifi = EspWifi::new(self.modem, self.sys_loop.clone(), None)?;
        let mut wifi = BlockingWifi::wrap(&mut esp_wifi, self.sys_loop)?;
        wifi.start()?;
        wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: self.credentials.ssid.parse().unwrap(),
            password: self.credentials.password.parse().unwrap(),
            channel: None,
            auth_method: self.credentials.auth_method,
            ..Default::default()
        }))?;
        wifi.connect()?;
        wifi.wait_netif_up()?;
        Ok(esp_wifi)
    }
}

#[macro_use]
mod network;
mod schedule;
mod time;

use crate::schedule::{Job, Scheduler};
use crate::time::remote::Request;
use anyhow::Result;
use chrono::{NaiveDateTime, ParseResult};
use embedded_svc::wifi::AuthMethod;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::prelude::Peripherals};
use network::wifi::{Connection, Credentials};
use dotenv_codegen::dotenv;
use time::remote::model::NinjasResponse;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let creds = Credentials::new(
        dotenv!("SSID").to_string(),
        dotenv!("PASS").to_string(),
        AuthMethod::WPA2Personal,
    );
    let connection = Connection::new(
        creds,
        Peripherals::take()?.modem,
        EspSystemEventLoop::take()?,
    );
    let _wifi = connection.init()?;
    let mut req = Request::new_https()?;
    let dt = req.time::<NinjasResponse>(
        "https://api.api-ninjas.com/v1/worldtime?lat=55.751244&lon=37.618423",
        vec![("X-Api-Key", dotenv!("API_KEY"))],
        |v| -> ParseResult<NaiveDateTime> {
            NaiveDateTime::parse_from_str(v.datetime.as_str(), "%Y-%m-%d %H:%M:%S")
        },
    )?;
    Scheduler::new(
        dt,
        vec![
            Job::new(10, || {
                println!("power on");
            }),
            Job::new(11, || {
                println!("power off");
            }),
        ],
    ).run();
    Ok(())
}

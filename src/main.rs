mod network;
mod time;
mod schedule;

use crate::time::remote::{Request};
use anyhow::Result;
use embedded_svc::wifi::AuthMethod;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::prelude::Peripherals,
};
use network::wifi::{Connection, Credentials};
use time::remote::model::NinjasResponse;
use chrono::{NaiveDateTime, ParseResult};
use crate::schedule::{Job, Scheduler};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let creds = Credentials::new(
        "ssid".parse()?,
        "pass".parse()?,
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
        vec![("X-Api-Key", "key")],
        |v| -> ParseResult<NaiveDateTime> {
            NaiveDateTime::parse_from_str(v.datetime.as_str(),"%Y-%m-%d %H:%M:%S")
        }
    )?;
    let scheduler = Scheduler::new(dt, vec![
        Job::new(9, || {
            println!("power on");
        }),
        Job::new(10, || {
            println!("power off");
        })
    ]);
    scheduler.run();
    Ok(())
}

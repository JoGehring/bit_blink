use std::error::Error;
use std::time::Duration;

use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio::time;
use uuid::Uuid;

pub use crate::bluetooth::message::{Animation, Message, Speed};

mod message;
pub mod utils;

const BADGE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFEE1);

pub async fn connection(input_message: &Message) -> Result<&'static str, Box<dyn Error>> {
    let handle = tokio::runtime::Handle::current();
    let enterGuard = handle.enter();
    let manager = Manager::new().await?;

    // get adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await.expect("scan failed");
    time::sleep(Duration::from_millis(400)).await;

    let mut badge_option: Option<Peripheral> = None;
    let mut counter1 = 0;
    while badge_option.is_none() {
        if counter1 > 6 { //stop trying if it didn't worked after some attempts
            return Err(Box::try_from("no badge found after ".to_owned() + &*counter1.to_string() + " attempts").unwrap());
        }
        badge_option = find_badge(&central).await;
        time::sleep(Duration::from_millis(400)).await;
        counter1 = counter1 + 1;
    }
    let badge = badge_option.unwrap();
    central.stop_scan();

    // connect to the device
    badge.connect().await?;
    time::sleep(Duration::from_millis(200)).await;

    let mut counter2 = 0;
    let mut badge_connected = false;
    while !badge_connected { //try to connect till the connection worked

        if counter2 > 6 { //stop trying if it didn't worked after some attempts
            return Err(Box::try_from("connection failed after ".to_owned() + &*counter2.to_string() + " attempts").unwrap());
        }
        badge.connect().await?;
        time::sleep(Duration::from_millis(200)).await;
        badge_connected = badge.is_connected().await?;
        time::sleep(Duration::from_millis(200)).await;
        counter2 = counter2 + 1;
    }

    // discover services
    badge.discover_services().await?;
    time::sleep(Duration::from_millis(200)).await;

    //get characteristics
    let chars = badge.characteristics();
    //shows all characteristics with which you can communicate with the led badge (badge.services() only shows the services)
    for characteristics in &chars {
        println!("    - {:?}", characteristics);
    }

    //find the right characteristic
    let cmd_char = chars
        .iter()
        .find(|c| {
            c.uuid == BADGE_CHARACTERISTIC_UUID
        })
        .expect("no characteristic found");

    let bluetooth_messages = input_message.build_bluetooth_message();


    for bluetooth_message in bluetooth_messages {
        badge.write(&cmd_char, &bluetooth_message, WriteType::WithoutResponse).await?;
    }

    badge.disconnect();

    Ok("Message sent")
}


async fn find_badge(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("LSLED"))
        {
            return Some(p);
        }
    }
    None
}


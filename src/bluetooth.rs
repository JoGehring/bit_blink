use btleplug::api::{bleuuid::uuid_from_u16, bleuuid::uuid_from_u32, Central, Manager as _, Peripheral as _, ScanFilter, WriteType, };
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::string;
use std::time::Duration;
use std::borrow::Borrow;
use tokio::time;
use uuid::Uuid;

const BADGE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFEE1);

pub async fn connection() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;

    // get adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    time::sleep(Duration::from_millis(800)).await;

    //find_all_devices(&central).await.expect("Error will searching for devices");
    let badge = find_badge(&central).await.expect("No badge found");

    // connect to the device
    badge.connect().await?;
    time::sleep(Duration::from_millis(200)).await;

    // discover services and characteristics
    badge.discover_services().await?;
    time::sleep(Duration::from_millis(200)).await;

    //zeigt alle characteristics an mit denen über UUID mit Badge kommuniziert werden kann (badge.services() zeigt services an)
    let chars = badge.characteristics();
    for characteristics in &chars {
        println!("    - {:?}", characteristics);
    }

    let cmd_char = chars
        .iter()
        .find(|c| {
            c.uuid == BADGE_CHARACTERISTIC_UUID
        })
        .expect("no characteristic found");

    let bluetooth_message1 = vec![0x77, 0x61, 0x6E, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let bluetooth_message2 = vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let bluetooth_message3 = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE1, 0x0C, 0x07, 0x00, 0x20, 0x31, 0x00, 0x00, 0x00, 0x00];
    let bluetooth_message4 = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let bluetooth_message5 = vec![0x00, 0x38, 0x6C, 0xC6, 0xC6, 0xFE, 0xC6, 0xC6, 0xC6, 0xC6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];  //00:38:6C:C6:C6:FE:C6:C6:C6:C6:00

    let bluetooth_messages = vec![&bluetooth_message1, &bluetooth_message2, &bluetooth_message3, &bluetooth_message4, &bluetooth_message5]; //buildBluetoothMessage();

    for bluetoothMessage in bluetooth_messages {
        badge.write(&cmd_char, &bluetoothMessage, WriteType::WithoutResponse).await?;
    }

    badge.disconnect();

    Ok(())
}

pub async fn find_badge(central: &Adapter) -> Option<Peripheral> {
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
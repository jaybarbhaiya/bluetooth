use std::{io, time::Duration};

use anyhow::{Ok, Result};
use bluez_async::BluetoothSession;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new session. This establishes the D-Bus connection to talk to BlueZ. In this case we
    // ignore the join handle, as we don't intend to run indefinitely.
    let (_, session) = BluetoothSession::new().await?;

    // Start scanning for Bluetooth devices, and wait a few seconds for some to be discovered.
    session.start_discovery().await?;
    time::sleep(Duration::from_secs(5)).await;
    session.stop_discovery().await?;

    // Get a list of devices which are currently known.
    let devices = session.get_devices().await?;

    for (index, device) in devices.iter().enumerate() {
        let name = match device.name.clone() {
            Some(name) => name,
            None => {
                let alias = device
                    .alias
                    .clone()
                    .unwrap_or_else(|| "No Deive".to_string());
                alias
            }
        };
        println!("{}. {}", index, name)
    }

    let choice = ask_choice();

    let selected_device = &devices[choice];
    session.connect(&selected_device.id).await?;

    // Find the device we care about.
    // let device = devices
    //     .into_iter()
    //     .find(|device| device.name.as_deref() == Some("My device"));

    // dbg!(device);

    Ok(())
}

fn ask_choice() -> usize {
    // Ask the user to choose the bluetooth device to connect.
    println!("Select device to connection (Enter device index number):");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to readline");
    choice.trim().parse().expect("Please type a number!")
}

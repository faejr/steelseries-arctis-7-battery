extern crate hidapi;

use hidapi::{HidApi, HidError};

fn main() -> Result<(), HidError> {
    let api = HidApi::new().expect("Failed to create API instance");

    let headset = api.open(0x1038, 0x12ad).expect("Failed to open device");

    let request: Vec<u8> = vec![0x06, 0x18];
    let mut len = headset.write(&request)?;

    if len == 0 {
        panic!("Write failure");
    }

    let mut data_read: [u8; 8] = [0; 8];
    len = headset.read(&mut data_read)?;

    if len == 0 {
        panic!("Read failure");
    }

    let level = data_read[2];

    println!("{}%", level);

    Ok(())
}
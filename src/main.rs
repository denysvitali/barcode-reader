extern crate serial;

mod barcodes;

use barcodes::barcode;
use barcodes::barcode::BarcodeType;

use std::io;
use std::io::ErrorKind;
use std::fmt::Write;

use std::time::Duration;
use serial::prelude::*;

fn main() {
    // Open Serial
    let mut port = serial::open("/dev/ttyACM0").unwrap();
    loop {
        match interact(&mut port) {
            Ok(_result) => {},
            Err(e) => {
                if e.kind() == ErrorKind::TimedOut {
                    println!("Timed Out");
                }
                else{
                    println!("Error Kind: {:?}", e.kind());
                }
            }
        }
    }
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()>{
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(100000))?;

    let mut result: Vec<u8> = Vec::new();
    let mut buf = vec![1, 50];

    let mut carriage_return = false;
    while !carriage_return {
        port.read(&mut buf)?;
        for x in &buf{
            if *x == '\r' as u8 {
                carriage_return = true;
                break;
            }
            result.push(*x);
        }
    }

    let barcode_type = barcode::from_char(char::from(*result.get(0).unwrap()));
    let mut s = String::new();
    println!("barcode type: {:?}", barcode_type);

    if barcode_type == BarcodeType::EAN13 || barcode_type == BarcodeType::Code128 {
        // Clean Barcode
        result.remove(0);
    }

    println!("Buf: {:?}", &result);
    for &byte in &result{
        write!(&mut s, "{:X} ", byte).expect("Unable to write");
    }

    println!("Hex: {}", s);

    s = match String::from_utf8(result) {
        Ok(t) => t,
        _ => String::from("N/A")
    };

    println!("barcode: {:?}", s);

    Ok(())
}

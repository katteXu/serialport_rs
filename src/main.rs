use std::fs::File;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use clap::{Arg, Command};

fn main() {
    let list = serialport::available_ports().unwrap();

    let port_name = "/dev/tty.usbmodem1201";

    let baud_rate = 9600;
    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 512];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(&mut serial_buf) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
                sleep(Duration::from_millis(20));

                if serial_buf.starts_with(&[2, 43]) && serial_buf.len() > 12 {
                    let weight = String::from_utf8_lossy(&serial_buf[2..8]).to_string();

                    let _weight = match weight.parse::<f32>() {
                        Ok(w) => w / 100.0,
                        Err(err) => 0.00,
                    };
                    println!("weight : {:.2?}", _weight);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

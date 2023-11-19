//#![allow(unused_imports, dead_code)]
use serialport::{available_ports, SerialPortType};

fn main() {
    match available_ports() {
        Ok(ports) => {
            match ports.len() {
                0 => println!("No ports found!!!"),
                1 => println!("Found one port"),
                n => println!("Found {} ports: ", n),
            };
            for p in ports {
                println!("  {}", p.port_name);
                match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("{:?}", info);
                    }
                    SerialPortType::BluetoothPort => {
                        println!("1");
                    }
                    SerialPortType::PciPort => {
                        println!("1");
                    }
                    SerialPortType::Unknown => {
                        println!("1");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}

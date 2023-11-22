// #![allow(unused_imports, dead_code)]
use sigrok::Sigrok;
use sigrok::Session;
use sigrok::data::{Datafeed, Logic};
use sigrok::config::{config_items, Configurable};
use std::error::Error;
use serialport::{available_ports, SerialPortType};

fn main()-> Result<(), Box<dyn Error>> {
    println!("list serial port");
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

    println!("list sigrok driver");
    let ctx = Sigrok::new()?;
    for driver in ctx.drivers() {
        println!("---- {:?}: {} v{}",
                 driver.name(),
                 driver.long_name(),
                 driver.api_version());
    }

    let ctx = Sigrok::new()?;
    let ses = Session::new(&ctx)?;

    let driver = ctx.drivers().into_iter().find(|x| x.name() == "demo").unwrap();
    let driver = driver.init()?;
    for device in driver.scan(None)? {
        ses.add_device(&device)?;
        device.config_set(config_items::LimitSamples, &64)?;

        if let Some(group) = device.channel_groups().get(0) {
            group.config_set(config_items::PatternMode, "sigrok")?;
        }

        device.config_set(config_items::SampleRate, &1_000_000)?;
    }

    ses.start(None, |_, data| match data {
        Datafeed::Logic(Logic { unit_size, data }) => {
            //let _ = unit_size;
            println!("unit_size {}", unit_size);
            for byte in data {
                println!("{}", format!("{:08b}", byte).replace("1", "-").replace("0", "+"));
            }
        }
        _ => {}
    })?;

    Ok(())
}

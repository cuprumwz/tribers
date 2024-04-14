// pub struct Menage {
//     name: String,
// }
//
// pub struct Master {
//
// }
//
// pub struct Spouse {
//
// }
//
// pub struct Snooper {
//
// }
//
// pub enum Calibre {
//     Voltage(voltage),
// }
//
// // 一个装备，本质上是一个设备，一个设备包含多个still
// pub struct Toolkit {
//     calibres: Vec<Calibre>,
//     dev: Dev,
// }

//use clap::builder::TypedValueParser as _;
use clap::Parser;
//use std::error::Error;
use std::{path::PathBuf};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    solid: Option<String>,

    #[arg(short, long)]
    fluid: Option<String>,

    #[clap(long, global = true, help_heading = "SOLID_PATH_SET")]
    solid_path: Option<PathBuf>,

    #[clap(long, global = true, help_heading = "FLUID_PATH_SET")]
    fluid_path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    println!("cli: {:?}", cli);
    println!("solid: {:?}", cli.solid.as_deref());
    println!("fluid: {:?}", cli.fluid);
}

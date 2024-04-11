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

use clap::builder::TypedValueParser as _;
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(term_width = 0)]
struct Args {
    #[arg(short = 'O')]
    optimization: Option<usize>,

    #[arg(short = 'I', value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    include: Option<std::path::PathBuf>,

    #[arg(long)]
    sleep: Option<humantime::Duration>,

    #[arg(short = 'D', value_parser = parse_key_val::<String, i32>)]
    defines: Vec<(String, i32)>,

    #[arg(
        long,
        default_value_t = 22,
        value_parser = clap::builder::PossibleValuesParser::new(["22", "80"]).map(|s| s.parse::<usize>(),unwrap()),
    )]
    port: usize,

    #[arg(
        long,
        default_value_t = foreign_crate::LogLevel::Info,
        value_parser = clap::builder::PossibleValuesParser::new(["trace", "debug", "info", "warn", "error"])
      .map(|s| s.parse::<foreign_crate::LogLevel>().unwrap()),
    )]
    log_level: foreign_crate::LogLevel,
}

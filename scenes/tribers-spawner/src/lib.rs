use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use serde::Deserialize;

use prost_wkt_types::*;

pub mod whisper {
	include!(concat!(env!("OUT_DIR"), "/whisper.rs"));
}

pub use whisper::*;

// 上电初始化一次的参数 'static
#[derive(Deserialize, Debug)]
pub struct Innate {
    pub shrine: PathBuf,        // spider数据库保存路径  assets.db

    pub fibase_valver: PathBuf,
    pub sibase_valver: PathBuf,
    pub fibase_snaper: PathBuf,
    pub sibase_snaper: PathBuf,
    pub sibase_larder: PathBuf,
    pub fibase_larder: PathBuf,
    pub sxmass_larder: PathBuf,
    pub fxmass_larder: PathBuf,
    pub sxplug_larder: PathBuf,
    pub fxplug_larder: PathBuf,

    pub surname: String,
    pub version: String,
    pub menages: Vec<Menage>,
}

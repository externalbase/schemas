use std::path::PathBuf;
use clap::{Parser};
use crate::WriteLang; 

#[derive(Parser, Debug)]
pub struct Cli {
    // 

    pub process: String,

    #[arg(value_enum)]
    pub format: WriteLang,

    // Options: 

    #[arg(short, long, default_value = "./output")]
    pub output: PathBuf,

    #[arg(short, long)]
    pub filters: Option<Vec<String>>,
}
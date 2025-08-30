mod raw_schema;
mod writers;
mod dumper;
mod cli;

use std::{fs::File, io::{Error, Write}, path::PathBuf, sync::RwLock};

use clap::{Parser, ValueEnum};
use exbase::*;
use crate::{cli::Cli, dumper::*, writers::*};

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";


pub static FILTERS: RwLock<Option<Vec<String>>> = RwLock::new(None);

fn main() -> Result<(), Error> {

    let cli = Cli::parse();
    let output_dir = &cli.output;

    if cli.filters.is_some() {
        let mut flt_guard = FILTERS.write().unwrap();
        *flt_guard = cli.filters;
    }

    if !std::fs::exists(output_dir)? {
        std::fs::create_dir(output_dir)?;

    }
    let proc = exbase::get_process_info_list(cli.process.clone())
        .unwrap()
        .into_iter()
        .next()
        .expect(&format!("'{}'not found", cli.process));

    let libschema = proc.get_modules()
        .unwrap()
        .into_iter()
        .find(|x| x.name().trim_start_matches("lib").trim_end_matches(".so").trim_end_matches(".dll") == "schemasystem")
        .expect("'libschemasystem.so or schemasystem.dll' not found");

    let mem = StreamMem::new(proc.pid()).unwrap();

    let mut schema = Schema::new(&mem, libschema);
    let scopes = schema.read_scopes();

    Ok(cli.format.write_file(&mem, &scopes, output_dir)?)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum WriteLang {
    #[value(name = "rs")]
    Rust,
    #[value(name = "cs")]
    CSharp
}

impl WriteLang {

    pub fn write_file(&self, mem: &impl MemoryAccessor, scopes: &Vec<TypeScope>, output_dir: &PathBuf) -> Result<(), Error> {
        for scope in scopes {
            let len = scope.classes.len();
            let file_name = &format!("{}/{}.{}", output_dir.display(), scope.name(), self.extension());
            if len > 0 {
                let mut file = File::create(file_name)?;
                self.write(mem, scope, &mut file)?;
                println!("{GREEN}{file_name}{RESET}");
            }
            else {
                println!("{file_name}");
            }
        }
        Ok(())
    }

    fn write<W: Write>(&self, mem: &impl MemoryAccessor, scope: &TypeScope, out: &mut W) -> Result<(), Error> {
        let mut ctx = Context::new(mem, scope, out);
        Ok(match self {
            WriteLang::Rust => RustModuleWriter::write_module(&mut ctx)?,
            WriteLang::CSharp => CSharpModuleWriter::write_module(&mut ctx)?,
        })
    }
    
    fn extension<'a>(&self) -> &'a str {
        match self {
            WriteLang::Rust => "rs",
            WriteLang::CSharp => "cs",
        }
    }
}
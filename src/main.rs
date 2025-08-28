mod raw_schema;
mod writers;
mod dumper;

use std::{fs::File, io::{Error, Write}};

use exbase::*;
use crate::{dumper::*, writers::*};

// Параметры
const PROCESS_NAME: &str = "dota2";

const OUTPUT_DIR: &str = "./output";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() -> Result<(), Error> {
    if !std::fs::exists(OUTPUT_DIR)? {
        std::fs::create_dir(OUTPUT_DIR)?;
    }
    let proc = exbase::get_process_info_list(PROCESS_NAME)
        .unwrap()
        .into_iter()
        .next()
        .expect("Process not found");

    let libschema = proc.get_modules()
        .unwrap()
        .into_iter()
        .find(|x| x.name().trim_start_matches("lib").trim_end_matches(".so").trim_end_matches(".dll") == "schemasystem")
        .expect("'libschemasystem.so or schemasystem.dll' not found");

    let mem = StreamMem::new(proc.pid()).unwrap();

    let mut schema = Schema::new(&mem, libschema);
    let scopes = schema.read_scopes();

    Ok(Lang::Rust.write_file(&mem, &scopes)?)
}

pub enum Lang {
    Rust,
    CSharp
}

impl Lang {

    pub fn write_file(&self, mem: &StreamMem, scopes: &Vec<TypeScope>) -> Result<(), Error> {
        for scope in scopes {
            let len = scope.classes.len();
            let file_name = &format!("{}/{}.{}", OUTPUT_DIR, scope.name(), self.extension());
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

    fn write<W: Write>(&self, mem: &StreamMem, scope: &TypeScope, out: &mut W) -> Result<(), Error> {
        let mut ctx = Context::new(mem, scope, out);
        Ok(match self {
            Lang::Rust => RustModuleWriter::write_module(&mut ctx)?,
            Lang::CSharp => CSharpModuleWriter::write_module(&mut ctx)?,
        })
    }
    
    fn extension<'a>(&self) -> &'a str {
        match self {
            Lang::Rust => "rs",
            Lang::CSharp => "cs",
        }
    }
}
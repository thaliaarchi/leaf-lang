use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;
use std::{env, io};

use leafy::{Program, VM};

fn main() {
    let mut args = env::args_os();
    if args.len() > 2 {
        let name = env::current_exe().ok();
        let name = name
            .as_ref()
            .map(Path::new)
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .unwrap_or("leaf");
        eprintln!("Usage: {name} [program]");
        process::exit(2);
    }

    let mut src = String::new();
    let res = if let Some(filename) = args.nth(1) {
        File::open(filename).and_then(|mut f| f.read_to_string(&mut src))
    } else {
        io::stdin().lock().read_to_string(&mut src)
    };
    if let Err(err) = res {
        eprintln!("{err}");
        process::exit(1);
    }

    let prog = match Program::parse(&src) {
        Ok(prog) => prog,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };
    let mut vm = VM::new(prog);
    if let Err(err) = vm.run() {
        eprintln!("{err}");
        process::exit(1);
    }
    print!("{}", vm.tree().dump_dot_to_string());
}

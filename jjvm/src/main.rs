use std::{
    collections::HashMap,
    fs,
    io::{self, Cursor},
};

use clap::{crate_authors, crate_version, App, Arg};
use jjvm_loader::class_loader::ClassLoader;
use jjvm_vm::{frame::Frame, heap::Heap, vm::VM};

fn main() {
    let matches = App::new("jjvm")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Rust-based JVM")
        .arg(
            Arg::with_name("INPUT")
                .help("The input file to use")
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("VM prints out each step"),
        )
        .get_matches();

    let input_files = matches.values_of("INPUT").unwrap().collect::<Vec<_>>();

    run_files(input_files, matches.is_present("debug")).unwrap();
}

fn run_files(input_files: Vec<&str>, debug: bool) -> Result<(), io::Error> {
    let mut classes = HashMap::new();
    let mut main_class = None;
    for file in input_files {
        let class = ClassLoader::new(Cursor::new(fs::read(file)?)).load();
        classes.insert(class.name.clone(), class.clone());
        if class.methods.iter().any(|f| f.name == "main") {
            main_class = Some(class.clone());
        }
    }

    let mut vm = VM {
        heap: Heap { heap: vec![] },
        classes,
        references: HashMap::new(),
        heap_last_gc_size: 4,
        should_gc: false,
        debug,
    };

    // let main_class = vm
    //     .classes
    //     .values()
    //     .find(|cls| cls.methods.iter().find(|m| m.name == "main").is_some())
    //     .clone()
    //     .expect("Could not find main class");

    let mut main_frame = Frame::from_method(
        &main_class.clone().expect("No main class"),
        "main".to_string(),
        vec![],
    )
    .unwrap();

    vm.exec(&main_class.unwrap(), &mut main_frame);

    Ok(())
}

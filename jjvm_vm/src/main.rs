use std::{collections::HashMap, fs, io::Cursor};

use jjvm_loader::class_loader::ClassLoader;
use jjvm_vm::{frame::Frame, heap::Heap, vm::VM};

use chrono::{DateTime, Utc};
use env_logger::Builder;
use std::io::Write;
use walkdir::WalkDir;

fn main() {
    configure_logging();

    let mut loader = ClassLoader::new(Cursor::new(fs::read("../Test.class").unwrap()));

    let class = loader.load();

    let mut add_loader = ClassLoader::new(Cursor::new(fs::read("../OtherTest.class").unwrap()));

    let add_class = add_loader.load();

    let mut vm = VM {
        heap: Heap { heap: vec![] },
        classes: HashMap::new(),
        references: HashMap::new(),
        heap_last_gc_size: 4,
        should_gc: false,
        debug: true,
    };

    for entry in WalkDir::new("../std")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.path().to_string_lossy().to_string();
        if !fs::metadata(f_name.clone()).unwrap().is_file() {
            continue;
        }
        println!("{}", f_name);
        let mut loader = ClassLoader::new(Cursor::new(fs::read(f_name).unwrap()));

        let class = loader.load();
        let name = class.name.clone();
        vm.classes.insert(name, class.clone());
    }

    vm.classes.insert("Test".to_string(), class.clone());
    vm.classes.insert("OtherTest".to_string(), add_class);

    let mut frame = Frame::from_method(&class, "main".to_string(), vec![]).unwrap();

    vm.exec(&class, &mut frame);
}

fn configure_logging() {
    let mut builder = Builder::from_default_env();
    builder.format(|buf, record| {
        let utc: DateTime<Utc> = Utc::now();

        write!(
            buf,
            "{:?} {} [{}] ",
            //utc.format("%Y-%m-%dT%H:%M:%S.%fZ"),
            utc, // same, probably faster?
            record.level(),
            record.target()
        )?;

        match (record.file(), record.line()) {
            (Some(file), Some(line)) => write!(buf, "[{}/{}] ", file, line),
            (Some(file), None) => write!(buf, "[{}] ", file),
            (None, Some(_line)) => write!(buf, " "),
            (None, None) => write!(buf, " "),
        }?;

        writeln!(buf, "{}", record.args())
    });

    builder.init();
}

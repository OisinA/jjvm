use std::{collections::HashMap, fs, io::Cursor};

use jjvm_loader::class_loader::ClassLoader;
use jjvm_vm::{frame::Frame, heap::Heap, vm::VM};

fn main() {
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
        debug: false,
    };

    vm.classes.insert("Test".to_string(), class.clone());
    vm.classes.insert("OtherTest".to_string(), add_class);

    let mut frame = Frame::from_method(&class, "main".to_string(), vec![]).unwrap();

    vm.exec(&class, &mut frame);

    println!("{:?}", vm.heap.heap.len());
}

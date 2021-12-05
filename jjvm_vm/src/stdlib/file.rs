use std::collections::HashMap;

use jjvm_loader::class::Field;

use crate::{jvm_val::JvmVal, vm::VM};

use super::class::BuiltinClass;

pub struct FileClass {}

impl BuiltinClass for FileClass {
    fn get_class_name(self) -> String {
        "java/io/File".to_string()
    }

    fn get_fields(&self) -> Vec<Field> {
        let fields = vec![Field {
            flags: 0x0001,
            name: "path".to_string(),
            descriptor: "Ljava/lang/String;".to_string(),
            attributes: vec![],
        }];

        return fields;
    }

    fn get_method(&self, method: String) -> fn(&mut VM, Vec<JvmVal>) -> JvmVal {
        match method.as_str() {
            "<init>" => FileClass::init,
            _ => panic!("method not found"),
        }
    }
}

impl FileClass {
    fn init(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let mut v = HashMap::new();

        v.insert("path".to_string(), vals[1].clone());

        let ptr = vm.heap.alloc(JvmVal::Class("java/io/File".to_string(), v));

        JvmVal::Reference(ptr)
    }
}

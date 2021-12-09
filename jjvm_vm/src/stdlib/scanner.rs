use std::{collections::HashMap, fs};

use jjvm_loader::class::Field;

use crate::{jvm_val::JvmVal, vm::VM};

use super::class::BuiltinClass;

pub struct ScannerClass {}

impl BuiltinClass for ScannerClass {
    fn get_class_name(self) -> String {
        "java/util/Scanner".to_string()
    }

    fn get_fields(&self) -> Vec<Field> {
        vec![Field {
            flags: 0x0001,
            name: "path".to_string(),
            descriptor: "Ljava/lang/String;".to_string(),
            attributes: vec![],
        }]
    }

    fn get_method(&self, method: String) -> fn(&mut VM, Vec<JvmVal>) -> JvmVal {
        match method.as_str() {
            "<init>" => ScannerClass::init,
            "hasNextLine" => ScannerClass::has_next_line,
            "nextLine" => ScannerClass::next_line,
            "close" => ScannerClass::close,
            _ => panic!("method not found {}", method),
        }
    }
}

impl ScannerClass {
    fn init(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let mut v = HashMap::new();

        let refer = match vals[1].clone() {
            JvmVal::Reference(v) => vm.heap.fetch(v),
            _ => panic!("expected reference"),
        };

        let cls = match refer {
            JvmVal::Class(_, vals) => vals.get("path").unwrap().clone(),
            _ => panic!("expected class"),
        };

        let path = match cls {
            JvmVal::String(c) => c,
            _ => panic!("expected string, got {:?}", cls),
        };

        let content = fs::read_to_string(path).unwrap();

        v.insert("file".to_string(), vals[0].clone());
        v.insert("line_pointer".to_string(), JvmVal::Int(0));
        v.insert("content".to_string(), JvmVal::String(content));

        let ptr = vm
            .heap
            .alloc(JvmVal::Class("java/util/Scanner".to_string(), v));

        JvmVal::Reference(ptr)
    }

    fn has_next_line(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let ptr = match vals[0].clone() {
            JvmVal::Reference(ptr) => ptr,
            _ => panic!("invalid argument"),
        };
        let scanner_values = match vm.heap.fetch(ptr) {
            JvmVal::Class(_, v) => v,
            _ => panic!("invalid argument"),
        };

        let line_pointer = match scanner_values.get(&"line_pointer".to_string()) {
            Some(JvmVal::Int(i)) => i,
            _ => panic!("invalid argument"),
        };

        let content = match scanner_values.get(&"content".to_string()) {
            Some(JvmVal::String(val)) => val,
            _ => panic!("invalid argument"),
        };

        JvmVal::Int(if *line_pointer < content.split('\n').count() as i32 {
            1
        } else {
            0
        })
    }

    fn next_line(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let ptr = match vals[0].clone() {
            JvmVal::Reference(ptr) => ptr,
            _ => panic!("invalid argument"),
        };

        let scanner = vm.heap.fetch_mut(ptr);

        let scanner_values = match scanner {
            JvmVal::Class(_, v) => v,
            _ => panic!("invalid argument"),
        };

        let cloned = scanner_values.clone();
        let line_pointer = match scanner_values.get_mut(&"line_pointer".to_string()) {
            Some(JvmVal::Int(i)) => {
                *i += 1;
                i
            }
            _ => panic!("invalid argument"),
        };

        let content = match cloned.get(&"content".to_string()) {
            Some(JvmVal::String(s)) => s,
            _ => panic!("invalid argument"),
        };

        JvmVal::String(
            content
                .split('\n')
                .nth(*line_pointer as usize - 1)
                .unwrap()
                .to_string(),
        )
    }

    fn close(_: &mut VM, _: Vec<JvmVal>) -> JvmVal {
        JvmVal::Null
    }
}

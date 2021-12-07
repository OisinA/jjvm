use std::collections::HashMap;

use jjvm_loader::class::Field;

use crate::{jvm_val::JvmVal, vm::VM};

use super::class::BuiltinClass;

pub struct BooleanClass {}

impl BuiltinClass for BooleanClass {
    fn get_class_name(self) -> String {
        "java/lang/Boolean".to_string()
    }

    fn get_fields(&self) -> Vec<Field> {
        let fields = vec![Field {
            flags: 0x0001,
            name: "value".to_string(),
            descriptor: "LI;".to_string(),
            attributes: vec![],
        }];

        return fields;
    }

    fn get_method(&self, method: String) -> fn(&mut VM, Vec<JvmVal>) -> JvmVal {
        match method.as_str() {
            "<init>" => BooleanClass::init,
            "valueOf" => BooleanClass::value_of,
            "booleanValue" => BooleanClass::boolean_value,
            _ => panic!("method not found {}", method),
        }
    }
}

impl BooleanClass {
    fn init(_: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        println!("{:?}", vals);
        JvmVal::Null
    }

    fn value_of(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let mut v = HashMap::new();

        v.insert("value".to_string(), vals[0].clone());

        let ptr = vm
            .heap
            .alloc(JvmVal::Class("java/lang/Boolean".to_string(), v));

        JvmVal::Reference(ptr)
    }

    fn boolean_value(vm: &mut VM, vals: Vec<JvmVal>) -> JvmVal {
        let mut v = HashMap::new();

        v.insert("value".to_string(), vals[0].clone());

        let boolean = vm.heap.fetch_mut(match vals[0].clone() {
            JvmVal::Reference(ptr) => ptr,
            _ => panic!("booleanValue expects a reference"),
        });

        let value = match boolean {
            JvmVal::Class(_, v) => v.get("value").unwrap().clone(),
            _ => panic!("booleanValue expects a reference"),
        };

        value
    }
}

use jjvm_loader::class::Field;

use crate::{jvm_val::JvmVal, vm::VM};

pub trait BuiltinClass {
    fn get_class_name(self) -> String;
    fn get_fields(&self) -> Vec<Field>;
    fn get_method(&self, method: String) -> fn(&mut VM, Vec<JvmVal>) -> JvmVal;
}

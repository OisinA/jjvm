use jjvm_loader::class::Field;

use crate::{jvm_val::JvmVal, vm::VM};

use super::class::BuiltinClass;

pub struct MathClass {}

impl BuiltinClass for MathClass {
    fn get_class_name(self) -> String {
        "java/lang/Math".to_string()
    }

    fn get_fields(&self) -> Vec<Field> {
        vec![]
    }

    fn get_method(&self, method: String) -> fn(&mut VM, Vec<JvmVal>) -> JvmVal {
        match method.as_str() {
            "abs" => MathClass::abs,
            "ceil" => MathClass::ceil,
            "floor" => MathClass::floor,
            "min" => MathClass::min,
            _ => panic!("method not found {}", method),
        }
    }
}

impl MathClass {
    fn abs(_: &mut VM, args: Vec<JvmVal>) -> JvmVal {
        let arg = args[0].clone();

        match arg {
            JvmVal::Int(i) => JvmVal::Int(i.abs()),
            JvmVal::Long(l) => JvmVal::Long(l.abs()),
            JvmVal::Float(f) => JvmVal::Float(f.abs()),
            JvmVal::Double(d) => JvmVal::Double(d.abs()),
            _ => panic!("unsupported type"),
        }
    }

    fn ceil(_: &mut VM, args: Vec<JvmVal>) -> JvmVal {
        let arg = args[0].clone();

        match arg {
            JvmVal::Float(f) => JvmVal::Float(f.ceil()),
            JvmVal::Double(d) => JvmVal::Double(d.ceil()),
            _ => panic!("unsupported type"),
        }
    }

    fn floor(_: &mut VM, args: Vec<JvmVal>) -> JvmVal {
        let arg = args[0].clone();

        match arg {
            JvmVal::Float(f) => JvmVal::Float(f.floor()),
            JvmVal::Double(d) => JvmVal::Double(d.floor()),
            _ => panic!("unsupported type"),
        }
    }

    fn min(_: &mut VM, args: Vec<JvmVal>) -> JvmVal {
        let arg = args[0].clone();

        match arg {
            JvmVal::Int(i) => {
                let mut min = i;

                for arg in args.iter() {
                    match arg {
                        JvmVal::Int(i) => {
                            if i < &min {
                                min = *i;
                            }
                        }
                        _ => panic!("unsupported type"),
                    }
                }

                JvmVal::Int(min)
            }
            JvmVal::Long(l) => {
                let mut min = l;

                for arg in args.iter() {
                    match arg {
                        JvmVal::Long(l) => {
                            if l < &min {
                                min = *l;
                            }
                        }
                        _ => panic!("unsupported type"),
                    }
                }

                JvmVal::Long(min)
            }
            JvmVal::Float(f) => {
                let mut min = f;

                for arg in args.iter() {
                    match arg {
                        JvmVal::Float(f) => {
                            if f < &min {
                                min = *f;
                            }
                        }
                        _ => panic!("unsupported type"),
                    }
                }

                JvmVal::Float(min)
            }
            JvmVal::Double(d) => {
                let mut min = d;

                for arg in args.iter() {
                    match arg {
                        JvmVal::Double(d) => {
                            if d < &min {
                                min = *d;
                            }
                        }
                        _ => panic!("unsupported type"),
                    }
                }

                JvmVal::Double(min)
            }
            _ => panic!("unsupported type"),
        }
    }
}

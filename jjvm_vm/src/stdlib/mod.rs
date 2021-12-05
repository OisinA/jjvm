use class::BuiltinClass;
use file::FileClass;

pub mod boolean;
pub mod class;
pub mod file;
pub mod integer;
pub mod scanner;

pub fn get_builtins(name: String) -> Box<dyn BuiltinClass> {
    match name.as_str() {
        "java/io/File" => Box::new(FileClass {}),
        "java/util/Scanner" => Box::new(scanner::ScannerClass {}),
        "java/lang/Boolean" => Box::new(boolean::BooleanClass {}),
        "java/lang/Integer" => Box::new(integer::IntegerClass {}),
        _ => panic!("Builtin class not found: {}", name),
    }
}

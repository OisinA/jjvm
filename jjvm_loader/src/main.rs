use std::{fs, io::Cursor};

use jjvm_loader::class_loader::ClassLoader;

use jjvm_loader::flags::ClassFlag;

fn main() {
    let mut loader = ClassLoader::new(Cursor::new(fs::read("SemanticdbVisitor.class").unwrap()));

    let class = loader.load();

    println!("Class: {}", class.name);
    println!("Superclass: {}", class.superclass);

    println!("Flags:");
    println!("\tPublic: {}", ClassFlag::Public.is_set(class.flags));
    println!("\tFinal: {}", ClassFlag::Final.is_set(class.flags));
    println!("\tSuper: {}", ClassFlag::Super.is_set(class.flags));
    println!("\tInterface: {}", ClassFlag::Interface.is_set(class.flags));
    println!("\tAbstract: {}", ClassFlag::Abstract.is_set(class.flags));
    println!("\tSynthetic: {}", ClassFlag::Synthetic.is_set(class.flags));
    println!(
        "\tAnnotation: {}",
        ClassFlag::Annotation.is_set(class.flags)
    );
    println!("\tEnum: {}", ClassFlag::Enum.is_set(class.flags));
    println!("\tModule: {}", ClassFlag::Module.is_set(class.flags));

    println!("Interfaces:");
    for interface in class.interfaces {
        println!("\tInterface: {}", interface);
    }

    println!("Fields:");
    for field in class.fields {
        println!("\tField: {}", field.name);
    }

    println!("Methods:");
    for method in class.methods {
        println!("\tMethod:");
        println!("\t\tName: {}", method.name);
        println!("\t\tAttributes:");
        for attribute in method.attributes {
            println!("\t\t\tAttribute:");
            println!("\t\t\t\tName: {}", attribute.name);
            if attribute.name == "Code" {
                println!("\t\t\t\tValue: ...");
            } else {
                println!("\t\t\t\tValue: {:?}", attribute.data);
            }
        }
    }

    println!("Attributes:");
    for attribute in class.attributes {
        println!("\tAttribute:");
        println!("\t\tName: {}", attribute.name);
        println!("\t\tValue: {:?}", attribute.data);
    }

    println!("{:?}", class.const_pool);
}

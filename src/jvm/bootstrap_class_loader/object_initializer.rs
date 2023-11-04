use std::collections::HashMap;

use crate::{
    class_loader::{
        attribute_info::AttributeInfo, class_file::ClassFile,
        constant_pool_info::ConstantPoolInfoType, parser::Parser,
    },
    jvm::{
        instructions::InstructionStream,
        object_manager::ObjectManager,
        types::{
            field::{Field, FieldAccessFlags},
            method::{Method, MethodAccesFlags},
            object::Object,
        },
    },
    resolve_constant,
};

pub fn initialize_class(mut class_file: ClassFile) -> Object {
    let name = get_name(&class_file);
    println!("Initializing class `{}`", name);
    let access_flags = get_access_flags(&class_file);
    let super_class = get_super_class(&class_file);
    let interfaces = get_interfaces(&mut class_file);
    let (fields, static_fields) = get_fields(&class_file);
    let methods = get_methods(&mut class_file);

    Object {
        name,
        access_flags,
        super_class,
        interfaces,
        fields,
        static_fields,
        methods,
        class_file,
    }
}

pub fn initialize_interface(mut class_file: ClassFile) -> Object {
    let name = get_name(&class_file);
    let access_flags = get_access_flags(&class_file);
    let super_class = get_super_class(&class_file);
    let interfaces = get_interfaces(&mut class_file);
    let (fields, static_fields) = get_fields(&class_file);
    let methods = get_methods(&mut class_file);

    Object {
        name,
        access_flags,
        super_class,
        interfaces,
        fields,
        static_fields,
        methods,
        class_file,
    }
}

fn get_name(class_file: &ClassFile) -> String {
    let class = resolve_constant!(
        ConstantPoolInfoType::Class,
        class_file.this_class,
        &class_file.constant_pool
    );
    resolve_constant!(
        ConstantPoolInfoType::Utf8,
        class.name_index,
        &class_file.constant_pool
    )
    .to_string()
}

fn get_access_flags(class_file: &ClassFile) -> u16 {
    class_file.access_flags
}

fn get_super_class(class_file: &ClassFile) -> Option<Box<Object>> {
    if class_file.super_class == 0 {
        None
    } else {
        let super_class = resolve_constant!(
            ConstantPoolInfoType::Class,
            class_file.super_class,
            &class_file.constant_pool
        );
        let name = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            super_class.name_index,
            &class_file.constant_pool
        )
        .to_string();
        let object = ObjectManager::get(name.as_str());
        Some(Box::new(object.clone()))
    }
}

fn get_interfaces(class_file: &mut ClassFile) -> HashMap<String, Object> {
    let mut interfaces: HashMap<String, Object> = HashMap::new();
    for interface in class_file.interfaces.drain(..) {
        let interface = resolve_constant!(
            ConstantPoolInfoType::Class,
            interface,
            &class_file.constant_pool
        );
        let name = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            interface.name_index,
            &class_file.constant_pool
        )
        .to_string();
        let object = ObjectManager::get(name.as_str());
        interfaces.insert(name.clone(), object.clone());
    }
    interfaces
}

fn get_fields(class_file: &ClassFile) -> (HashMap<String, Field>, HashMap<String, Field>) {
    let mut fields = HashMap::new();
    let mut static_fields = HashMap::new();

    for field in class_file.fields.iter() {
        let name = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            field.name_index,
            &class_file.constant_pool
        )
        .to_string();
        let descriptor = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            field.descriptor_index,
            &class_file.constant_pool
        )
        .to_string();
        let access_flags = field.access_flags;

        let value = None;

        if access_flags & FieldAccessFlags::Static as u16 != 0 {
            static_fields.insert(
                format!("{}{}", name.clone(), descriptor.clone()),
                Field::new(name, descriptor, access_flags, value),
            );
        } else {
            fields.insert(
                format!("{}{}", name.clone(), descriptor.clone()),
                Field::new(name, descriptor, access_flags, value),
            );
        }
    }
    (fields, static_fields)
}

fn get_methods(class_file: &mut ClassFile) -> HashMap<String, Method> {
    let mut methods = HashMap::new();

    for (_, method) in class_file.methods.iter().enumerate() {
        let name = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            method.name_index,
            &class_file.constant_pool
        )
        .to_string();

        let descriptor = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            method.descriptor_index,
            &class_file.constant_pool
        )
        .to_string();
        let access_flags = method.access_flags;

        if access_flags & MethodAccesFlags::Native as u16 != 0
            || access_flags & MethodAccesFlags::Abstract as u16 != 0
        {
            methods.insert(
                format!("{}{}", name.clone(), descriptor.clone()),
                Method::new(
                    name,
                    descriptor,
                    access_flags,
                    0,
                    0,
                    InstructionStream::new_native(),
                    true,
                ),
            );
        } else {
            let attr_code = match method
                .attributes
                .iter()
                .find(|attribute| matches!(attribute, AttributeInfo::Code(_)))
            {
                Some(AttributeInfo::Code(attr_code)) => attr_code,
                _ => panic!("Method `{}` does not have a Code attribute!", name),
            };
            let exception_handler_table = attr_code.get_exception_handler_table();
            let mut code_parser = Parser::new(attr_code.code.clone());
            let instruction_stream = InstructionStream::new(
                &mut code_parser,
                attr_code.code_length,
                exception_handler_table,
            );

            methods.insert(
                format!("{}{}", name.clone(), descriptor.clone()),
                Method::new(
                    name,
                    descriptor,
                    access_flags,
                    attr_code.max_locals,
                    attr_code.max_stack,
                    instruction_stream,
                    false,
                ),
            );
        }
    }
    methods
}

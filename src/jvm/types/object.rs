use std::collections::HashMap;

use crate::{class_loader::class_file::ClassFile, jvm::object_manager::ObjectManager};

use super::{field::Field, method::Method};

#[repr(u16)]
pub enum ObjectAccessFlags {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
    Module = 0x8000,
}

#[derive(Clone)]
pub struct Object {
    name: String,
    access_flags: u16,
    super_class: Option<Box<Object>>,
    interfaces: HashMap<String, Object>,
    fields: HashMap<String, Field>,
    static_fields: HashMap<String, Field>,
    methods: HashMap<String, Method>,
    class_file: ClassFile,
}

impl Object {
    pub fn new_class(
        name: String,
        access_flags: u16,
        super_class: Option<Box<Object>>,
        interfaces: HashMap<String, Object>,
        fields: HashMap<String, Field>,
        static_fields: HashMap<String, Field>,
        methods: HashMap<String, Method>,
        class_file: ClassFile,
    ) -> Self {
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

    pub fn new_interface(
        name: String,
        access_flags: u16,
        super_class: Option<Box<Object>>,
        interfaces: HashMap<String, Object>,
        static_fields: HashMap<String, Field>,
        methods: HashMap<String, Method>,
        class_file: ClassFile,
    ) -> Self {
        Object {
            name,
            access_flags,
            super_class,
            interfaces,
            fields: HashMap::with_capacity(0),
            static_fields,
            methods,
            class_file,
        }
    }

    pub fn get_method(&self, method_name: &str, descriptor: &str) -> Option<(&Self, &Method)> {
        let key = format!("{}{}", method_name, descriptor);
        if self.methods.contains_key(key.as_str()) {
            Some((self, self.methods.get(key.as_str()).unwrap()))
        } else if let Some(super_class) = &self.super_class {
            super_class.get_method(method_name, descriptor)
        } else {
            None
        }
    }

    pub fn put_static_field(&mut self, field: Field) {
        let key = format!("{}{}", field.name(), field.descriptor());
        self.static_fields.insert(key, field);
    }

    pub fn get_static_field(&mut self, field_name: &str, descriptor: &str) -> Option<&mut Field> {
        let key = format!("{}{}", field_name, descriptor);
        if self.static_fields.contains_key(key.as_str()) {
            self.static_fields.get_mut(key.as_str())
        } else if let Some(super_class) = &self.super_class {
            let super_class = ObjectManager::get(&super_class.name);
            super_class.get_static_field(field_name, descriptor)
        } else {
            None
        }
    }

    pub fn put_field(&mut self, field: Field) {
        let key = format!("{}{}", field.name(), field.descriptor());
        self.fields.insert(key, field);
    }

    pub fn get_field(&self, field_name: &str, descriptor: &str) -> Option<&Field> {
        let key: String = format!("{}{}", field_name, descriptor);
        if self.fields.contains_key(key.as_str()) {
            self.fields.get(key.as_str())
        } else if let Some(super_class) = &self.super_class {
            super_class.get_field(field_name, descriptor)
        } else {
            None
        }
    }

    pub fn is_class(&self) -> bool {
        !self.is_interface()
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Public as u16 != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Final as u16 != 0
    }

    pub fn is_super(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Super as u16 != 0
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Interface as u16 != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Abstract as u16 != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Synthetic as u16 != 0
    }

    pub fn is_annotation(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Annotation as u16 != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Enum as u16 != 0
    }

    pub fn is_module(&self) -> bool {
        self.access_flags & ObjectAccessFlags::Module as u16 != 0
    }

    pub fn has_main_method(&self) -> bool {
        self.methods.contains_key("main")
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn access_flags(&self) -> &u16 {
        &self.access_flags
    }

    pub fn super_class(&self) -> &Option<Box<Object>> {
        &self.super_class
    }

    pub fn interfaces(&self) -> &HashMap<String, Object> {
        &self.interfaces
    }

    pub fn fields(&self) -> &HashMap<String, Field> {
        &self.fields
    }

    pub fn static_fields(&self) -> &HashMap<String, Field> {
        &self.static_fields
    }

    pub fn methods(&self) -> &HashMap<String, Method> {
        &self.methods
    }

    pub fn class_file(&self) -> &ClassFile {
        &self.class_file
    }
}

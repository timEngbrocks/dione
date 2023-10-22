use std::collections::{HashSet, HashMap};

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

pub struct Object {
    pub name: String,
    pub access_flags: u16,
    pub super_class: Option<String>,
    pub interfaces: HashSet<String>,
    pub fields: HashMap<String, Field>,
    pub static_fields: HashMap<String, Field>,
    pub methods: HashMap<String, Method>,
}

impl Object {
    pub fn new_class(
        name: String,
        access_flags: u16,
        super_class: Option<String>,
        interfaces: HashSet<String>,
        fields: HashMap<String, Field>,
        static_fields: HashMap<String, Field>,
        methods: HashMap<String, Method>,
    ) -> Self {
        Object {
            name,
            access_flags,
            super_class,
            interfaces,
            fields,
            static_fields,
            methods,
        }
    }

    pub fn new_interface(
        name: String,
        access_flags: u16,
        super_class: Option<String>,
        interfaces: HashSet<String>,
        static_fields: HashMap<String, Field>,
        methods: HashMap<String, Method>,
    ) -> Self {
        Object {
            name,
            access_flags,
            super_class,
            interfaces,
            fields: HashMap::with_capacity(0),
            static_fields,
            methods,
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
}

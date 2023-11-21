use std::cmp::Ordering;

use super::{
    attribute_info::AttributeInfo,
    constant_pool_info::{ConstantPool, ConstantPoolInfo, ConstantPoolInfoType},
    field_info::FieldInfo,
    method_info::MethodInfo,
    parser::{Parser, U2, U4},
};
use crate::class_loader::{attribute_info::Attribute, constant_pool_info::ConstantEmptyItem};

pub enum ClassFileAccessFlags {
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

pub static mut GLOBAL_CLASS_FILE_MAJOR_VERSION: u16 = 0xFF;
pub static mut GLOBAL_CLASS_FILE_MINOR_VERSION: u16 = 0xFF;

pub fn compare_class_file_version_to_global(major_version: u16, minor_version: u16) -> Ordering {
    let global_major_version = unsafe { GLOBAL_CLASS_FILE_MAJOR_VERSION };
    let global_minor_version = unsafe { GLOBAL_CLASS_FILE_MINOR_VERSION };
    if major_version > global_major_version {
        Ordering::Greater
    } else if major_version < global_major_version {
        Ordering::Less
    } else if minor_version > global_minor_version {
        Ordering::Greater
    } else if minor_version < global_minor_version {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

#[derive(Debug, Clone)]
pub struct ClassFile {
    magic: U4,
    minor_version: U2,
    major_version: U2,
    constant_pool_count: U2,
    constant_pool: ConstantPool,
    access_flags: U2,
    this_class: U2,
    super_class: U2,
    interfaces_count: U2,
    interfaces: Vec<U2>,
    fields_count: U2,
    fields: Vec<FieldInfo>,
    methods_count: U2,
    methods: Vec<MethodInfo>,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn parse(data: Vec<u8>) -> ClassFile {
        let mut parser = Parser::new(data);
        let magic = parser.consume_u4();
        let minor_version = parser.consume_u2();
        let major_version = parser.consume_u2();

        unsafe {
            if GLOBAL_CLASS_FILE_MAJOR_VERSION == 0xFF {
                GLOBAL_CLASS_FILE_MAJOR_VERSION = major_version;
            }
            if GLOBAL_CLASS_FILE_MINOR_VERSION == 0xFF {
                GLOBAL_CLASS_FILE_MINOR_VERSION = minor_version;
            }
        }

        let constant_pool_count = parser.consume_u2();
        let mut constants: Vec<ConstantPoolInfoType> =
            Vec::with_capacity((constant_pool_count - 1) as usize);
        for _ in 0..(constant_pool_count - 1) {
            if let Some(c) = constants.last() {
                if c.get_tag() == &5 || c.get_tag() == &6 {
                    constants.push(ConstantPoolInfoType::EmptyItem(ConstantEmptyItem::new(
                        &mut parser,
                    )));
                    continue;
                }
            }
            constants.push(ConstantPoolInfoType::new(&mut parser));
        }
        let constant_pool = ConstantPool::new(constants);
        let access_flags = parser.consume_u2();
        let this_class = parser.consume_u2();
        let super_class = parser.consume_u2();
        let interfaces_count = parser.consume_u2();
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(parser.consume_u2());
        }
        let fields_count = parser.consume_u2();
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(FieldInfo::new(&mut parser, &constant_pool));
        }
        let methods_count = parser.consume_u2();
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(MethodInfo::new(&mut parser, &constant_pool));
        }
        let attributes_count = parser.consume_u2();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(&mut parser, &constant_pool));
        }

        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        }
    }

    pub fn is_class(&self) -> bool {
        !self.is_interface()
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags & ClassFileAccessFlags::Interface as u16 != 0
    }

    pub fn magic(&self) -> &U4 {
        &self.magic
    }

    pub fn minor_version(&self) -> &U2 {
        &self.minor_version
    }

    pub fn major_version(&self) -> &U2 {
        &self.major_version
    }

    pub fn constant_pool_count(&self) -> &U2 {
        &self.constant_pool_count
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    pub fn access_flags(&self) -> &U2 {
        &self.access_flags
    }

    pub fn this_class(&self) -> &U2 {
        &self.this_class
    }

    pub fn super_class(&self) -> &U2 {
        &self.super_class
    }

    pub fn interfaces_count(&self) -> &U2 {
        &self.interfaces_count
    }

    pub fn interfaces(&self) -> &Vec<U2> {
        &self.interfaces
    }

    pub fn fields_count(&self) -> &U2 {
        &self.fields_count
    }

    pub fn fields(&self) -> &Vec<FieldInfo> {
        &self.fields
    }

    pub fn methods_count(&self) -> &U2 {
        &self.methods_count
    }

    pub fn methods(&self) -> &Vec<MethodInfo> {
        &self.methods
    }

    pub fn attributes_count(&self) -> &U2 {
        &self.attributes_count
    }

    pub fn attributes(&self) -> &Vec<AttributeInfo> {
        &self.attributes
    }
}

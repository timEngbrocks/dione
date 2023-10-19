use super::{constant_pool_info::{ConstantPool, ConstantPoolInfo, ConstantPoolInfoType}, field_info::FieldInfo, method_info::MethodInfo, attribute_info::AttributeInfo, parser::{Parser, U2, U4}};
use crate::class_loader::{attribute_info::Attribute, constant_pool_info::ConstantEmptyItem};

#[derive(Debug)]
pub struct ClassFile {
	pub magic: U4,
    pub minor_version: U2,
    pub major_version: U2,
    pub constant_pool_count: U2,
    pub constant_pool: ConstantPool,
    pub access_flags: U2,
    pub this_class: U2,
    pub super_class: U2,
    pub interfaces_count: U2,
    pub interfaces: Vec<U2>,
    pub fields_count: U2,
    pub fields: Vec<FieldInfo>,
    pub methods_count: U2,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: U2,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn parse(data: Vec<u8>) -> ClassFile {
        let mut parser = Parser::new(data);
        let magic = parser.consume_u4();
        let minor_version = parser.consume_u2();
        let major_version = parser.consume_u2();
        let constant_pool_count = parser.consume_u2();
        let mut constants: Vec<ConstantPoolInfoType> = Vec::with_capacity((constant_pool_count - 1) as usize);
        for _ in 0..(constant_pool_count - 1) {
            match constants.last() {
                Some(c) => {
                    if c.get_tag() == &5 || c.get_tag() == &6 {
                        constants.push(ConstantPoolInfoType::EmptyItem(ConstantEmptyItem::new(&mut parser)));
                        continue;
                    }
                },
                None => {},
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
            attributes
        }
    }
}
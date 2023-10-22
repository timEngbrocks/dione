use std::collections::{HashSet, HashMap};

use crate::{class_loader::{class_file::ClassFile, constant_pool_info::ConstantPoolInfoType, attribute_info::{AttributeInfo, attribute_code::AttributeCode}, parser::Parser}, jvm::{types::{object::Object, field::{Field, FieldAccessFlags}, method::{Method, MethodAccesFlags}}, instructions::InstructionStream}, resolve_constant};

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
	}
}

fn get_name(class_file: &ClassFile) -> String {
	let class = resolve_constant!(ConstantPoolInfoType::Class, class_file.this_class, &class_file.constant_pool);
	resolve_constant!(ConstantPoolInfoType::Utf8, class.name_index, &class_file.constant_pool).to_string()
}

fn get_access_flags(class_file: &ClassFile) -> u16 {
	class_file.access_flags
}

fn get_super_class(class_file: &ClassFile) -> Option<String> {
	if class_file.super_class == 0 {
		None
	} else {
		let super_class = resolve_constant!(ConstantPoolInfoType::Class, class_file.super_class, &class_file.constant_pool);
		Some(resolve_constant!(ConstantPoolInfoType::Utf8, super_class.name_index, &class_file.constant_pool).to_string())
	}
}

fn get_interfaces(class_file: &mut ClassFile) -> HashSet<String> {
	let mut interfaces = HashSet::new();
	for interface in class_file.interfaces.drain(..) {
		let interface = resolve_constant!(ConstantPoolInfoType::Class, interface, &class_file.constant_pool);
		interfaces.insert(resolve_constant!(ConstantPoolInfoType::Utf8, interface.name_index, &class_file.constant_pool).to_string());
	}
	interfaces
}

fn get_fields(class_file: &ClassFile) -> (HashMap<String, Field>, HashMap<String, Field>) {
	let mut fields = HashMap::new();
	let mut static_fields = HashMap::new();

	for field in class_file.fields.iter() {
		let name = resolve_constant!(ConstantPoolInfoType::Utf8, field.name_index, &class_file.constant_pool).to_string();
		let descriptor = resolve_constant!(ConstantPoolInfoType::Utf8, field.descriptor_index, &class_file.constant_pool).to_string();
		let access_flags = field.access_flags;
		
		let value = None;

		if access_flags & FieldAccessFlags::Static as u16 != 0 {
			static_fields.insert(name.clone(), Field::new(
				name,
				descriptor,
				access_flags,
				value,
			));
		} else {
			fields.insert(name.clone(), Field::new(
				name,
				descriptor,
				access_flags,
				value,
			));
		}
	}
	(fields, static_fields)
}

fn get_methods(class_file: &mut ClassFile) -> HashMap<String, Method> {
	let mut methods = HashMap::new();
	for method in class_file.methods.iter() {
		let name = resolve_constant!(ConstantPoolInfoType::Utf8, method.name_index, &class_file.constant_pool).to_string();
		println!("Method `{}`", name);

		let descriptor = resolve_constant!(ConstantPoolInfoType::Utf8, method.descriptor_index, &class_file.constant_pool).to_string();
		let access_flags = method.access_flags;
		
		if access_flags & MethodAccesFlags::Native as u16 != 0 {
			methods.insert(name.clone(), Method::new(
				name,
				descriptor,
				access_flags,
				0,
				0,
				InstructionStream::new_native(&class_file.constant_pool),
				true,
			));
		} else {
			let (max_locals, max_stack, code_length, code) = match method.attributes.iter().find(|attribute| {
				match attribute {
					AttributeInfo::Code { .. } => true,
					_ => false,
				}
			}) {
				Some(AttributeInfo::Code(AttributeCode { max_locals, max_stack, code_length, code, .. })) => (max_locals, max_stack, code_length, code.clone()),
				_ => panic!("Method `{}` does not have a Code attribute!", name),
			};
			let mut code_parser = Parser::new(code);
			let instruction_stream = InstructionStream::new(
				&mut code_parser,
				code_length.clone(),
				max_locals.clone(),
				max_stack.clone(),
				&class_file.constant_pool,
			);

			methods.insert(name.clone(), Method::new(
				name,
				descriptor,
				access_flags,
				max_locals.clone(),
				max_stack.clone(),
				instruction_stream,
				false,
			));
		}
	}
	methods
}

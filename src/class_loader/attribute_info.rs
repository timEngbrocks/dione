use self::{attribute_constant_value::AttributeConstantValue, attribute_code::AttributeCode, attribute_stack_map_table::AttributeStackMapTable, attribute_bootstrap_methods::AttributeBootstrapMethods, attribute_nest_host::AttributeNestHost, attribute_nest_members::AttributeNestMembers, attribute_permitted_subclasses::AttributePermittedSubclasses, attribute_line_number_table::AttributeLineNumberTable, attribute_source_file::AttributeSourceFile, attribute_local_variable_table::AttributeLocalVariableTable, attribute_inner_classes::AttributeInnerClasses, attribute_exceptions::AttributeExceptions, attribute_local_variable_type_table::AttributeLocalVariableTypeTable, attribute_signature::AttributeSignature, attribute_runtime_visibile_annotations::AttributeRuntimeVisibleAnnotations, attribute_deprecated::AttributeDeprecated, attribute_record::AttributeRecord, attribute_method_parameters::AttributeMethodParameters, attribute_enclosing_method::AttributeEnclosingMethod, attribute_annotation_default::AttributeAnnotationDefault, attribute_runtime_invisible_annotations::AttributeRuntimeInvisibleAnnotations};

use super::{parser::Parser, constant_pool_info::{ConstantPool, ConstantPoolInfoType}};

pub mod attribute_constant_value;
pub mod attribute_code;
pub mod attribute_stack_map_table;
pub mod attribute_bootstrap_methods;
pub mod attribute_nest_host;
pub mod attribute_nest_members;
pub mod attribute_permitted_subclasses;
pub mod attribute_line_number_table;
pub mod attribute_source_file;
pub mod attribute_local_variable_table;
pub mod attribute_inner_classes;
pub mod attribute_exceptions;
pub mod attribute_runtime_visibile_annotations;
pub mod attribute_local_variable_type_table;
pub mod attribute_signature;
pub mod attribute_deprecated;
pub mod attribute_record;
pub mod attribute_method_parameters;
pub mod attribute_enclosing_method;
pub mod attribute_annotation_default;
pub mod attribute_runtime_invisible_annotations;

#[derive(Debug)]
pub enum AttributeInfo {
	ConstantValue(AttributeConstantValue),
	Code(AttributeCode),
	StackMapTable(AttributeStackMapTable),
	BootstrapMethods(AttributeBootstrapMethods),
	NestHost(AttributeNestHost),
	NestMembers(AttributeNestMembers),
	PermittedSubclasses(AttributePermittedSubclasses),
	LineNumberTable(AttributeLineNumberTable),
	SourceFile(AttributeSourceFile),
	LocalVariableTable(AttributeLocalVariableTable),
	InnerClasses(AttributeInnerClasses),
	Exceptions(AttributeExceptions),
	RuntimeVisibleAnnotations(AttributeRuntimeVisibleAnnotations),
	LocalVariableTypeTable(AttributeLocalVariableTypeTable),
	Signature(AttributeSignature),
	Deprecated(AttributeDeprecated),
	Record(AttributeRecord),
	MethodParameters(AttributeMethodParameters),
	EnclosingMethod(AttributeEnclosingMethod),
	AnnotationDefault(AttributeAnnotationDefault),
	RuntimeInvisibleAnnotations(AttributeRuntimeInvisibleAnnotations),
}

pub trait Attribute {
	fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> Self;
}

impl Attribute for AttributeInfo {
	fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> Self {
		let attribute_name_index = parser.peek_u2();
		let attribute_name = match constant_pool.get(attribute_name_index) {
			ConstantPoolInfoType::Utf8(info) => {
				match String::from_utf8(info.bytes.clone()) {
					Ok(v) => v,
					Err(error) => panic!("{error}")
				}
			},
			_ => panic!("Error handling!")
		};
		match attribute_name.as_str() {
			"ConstantValue" => AttributeInfo::ConstantValue(AttributeConstantValue::new(parser, constant_pool)),
			"Code" => AttributeInfo::Code(AttributeCode::new(parser, constant_pool)),
			"StackMapTable" => AttributeInfo::StackMapTable(AttributeStackMapTable::new(parser, constant_pool)),
			"BootstrapMethods" => AttributeInfo::BootstrapMethods(AttributeBootstrapMethods::new(parser, constant_pool)),
			"NestHost" => AttributeInfo::NestHost(AttributeNestHost::new(parser, constant_pool)),
			"NestMembers" => AttributeInfo::NestMembers(AttributeNestMembers::new(parser, constant_pool)),
			"PermittedSubclasses" => AttributeInfo::PermittedSubclasses(AttributePermittedSubclasses::new(parser, constant_pool)),
			"LineNumberTable" => AttributeInfo::LineNumberTable(AttributeLineNumberTable::new(parser, constant_pool)),
			"SourceFile" => AttributeInfo::SourceFile(AttributeSourceFile::new(parser, constant_pool)),
			"LocalVariableTable" => AttributeInfo::LocalVariableTable(AttributeLocalVariableTable::new(parser, constant_pool)),
			"InnerClasses" => AttributeInfo::InnerClasses(AttributeInnerClasses::new(parser, constant_pool)),
			"Exceptions" => AttributeInfo::Exceptions(AttributeExceptions::new(parser, constant_pool)),
			"RuntimeVisibleAnnotations" => AttributeInfo::RuntimeVisibleAnnotations(AttributeRuntimeVisibleAnnotations::new(parser, constant_pool)),
			"LocalVariableTypeTable" => AttributeInfo::LocalVariableTypeTable(AttributeLocalVariableTypeTable::new(parser, constant_pool)),
			"Signature" => AttributeInfo::Signature(AttributeSignature::new(parser, constant_pool)),
			"Deprecated" => AttributeInfo::Deprecated(AttributeDeprecated::new(parser, constant_pool)),
			"Record" => AttributeInfo::Record(AttributeRecord::new(parser, constant_pool)),
			"MethodParameters" => AttributeInfo::MethodParameters(AttributeMethodParameters::new(parser, constant_pool)),
			"EnclosingMethod" => AttributeInfo::EnclosingMethod(AttributeEnclosingMethod::new(parser, constant_pool)),
			"AnnotationDefault" => AttributeInfo::AnnotationDefault(AttributeAnnotationDefault::new(parser, constant_pool)),
			"RuntimeInvisibleAnnotations" => AttributeInfo::RuntimeInvisibleAnnotations(AttributeRuntimeInvisibleAnnotations::new(parser, constant_pool)),
			name => {
				println!("Unimplemented Attribute: {name}");
				parser.consume_u2();
				let attribute_length = parser.consume_u4();
				for _ in 0..attribute_length {
					parser.consume_u1();
				}
				AttributeInfo::ConstantValue(AttributeConstantValue {
					attribute_name_index: 0,
					attribute_length: 2,
					constantvalue_index: 0
				})
			}
		}
	}
}

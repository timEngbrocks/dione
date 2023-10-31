use crate::class_loader::parser::{Parser, U2};

use super::{ConstantPoolInfo, super::parser::U1};

pub enum ConstantMethodHandleKind {
	GetField,
	GetStatic,
	PutField,
	PutStatic,
	InvokeVirtual,
	InvokeStatic,
	InvokeSpecial,
	NewInvokeSpecial,
	InvokeInterface,
}

impl ConstantMethodHandleKind {
	pub fn from_kind(kind: u8) -> Self {
		match kind {
			1 => ConstantMethodHandleKind::GetField,
			2 => ConstantMethodHandleKind::GetStatic,
			3 => ConstantMethodHandleKind::PutField,
			4 => ConstantMethodHandleKind::PutStatic,
			5 => ConstantMethodHandleKind::InvokeVirtual,
			6 => ConstantMethodHandleKind::InvokeStatic,
			7 => ConstantMethodHandleKind::InvokeSpecial,
			8 => ConstantMethodHandleKind::NewInvokeSpecial,
			9 => ConstantMethodHandleKind::InvokeInterface,
			_ => panic!("Unknown method handle kind: {kind}"),
		}
	}

	pub fn get_kind(&self) -> u8 {
		match self {
			ConstantMethodHandleKind::GetField => 1,
			ConstantMethodHandleKind::GetStatic => 2,
			ConstantMethodHandleKind::PutField => 3,
			ConstantMethodHandleKind::PutStatic => 4,
			ConstantMethodHandleKind::InvokeVirtual => 5,
			ConstantMethodHandleKind::InvokeStatic => 6,
			ConstantMethodHandleKind::InvokeSpecial => 7,
			ConstantMethodHandleKind::NewInvokeSpecial => 8,
			ConstantMethodHandleKind::InvokeInterface => 9,
		}
	}
}

#[derive(Clone)]
pub struct ConstantMethodHandleInfo {
	pub tag: U1,
	pub reference_kind: U1,
	pub reference_index: U2,
}

impl ConstantPoolInfo for ConstantMethodHandleInfo {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let reference_kind = parser.consume_u1();
		let reference_index = parser.consume_u2();
		ConstantMethodHandleInfo { tag, reference_kind, reference_index }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}
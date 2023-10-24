use crate::class_loader::parser::U2;

use self::{byte::Byte, short::Short, int::Int, long::Long, char::Char, float::Float, double::Double, boolean::Boolean, return_address::ReturnAddress, reference::Reference, array::Array, object::Object};

pub mod byte;
pub mod short;
pub mod int;
pub mod long;
pub mod char;
pub mod float;
pub mod double;
pub mod boolean;
pub mod return_address;
pub mod reference;

pub mod object;
pub mod array;
pub mod field;
pub mod method;

pub enum Types {
	Byte(Byte),
	Short(Short),
	Int(Int),
	Long(Long),
	Char(Char),
	Float(Float),
	Double(Double),
	Boolean(Boolean),
	ReturnAddress(ReturnAddress),
	Reference(Reference),
}

#[derive(Clone)]
pub enum ReturnTypes {
	Type(Types),
	Void,
}

impl Clone for Types {
	fn clone(&self) -> Self {
		match self {
			Types::Byte(byte) => Types::Byte(byte.clone()),
			Types::Short(short) => Types::Short(short.clone()),
			Types::Int(int) => Types::Int(int.clone()),
			Types::Long(long) => Types::Long(long.clone()),
			Types::Char(char) => Types::Char(char.clone()),
			Types::Float(float) => Types::Float(float.clone()),
			Types::Double(double) => Types::Double(double.clone()),
			Types::Boolean(boolean) => Types::Boolean(boolean.clone()),
			Types::ReturnAddress(return_address) => Types::ReturnAddress(return_address.clone()),
			Types::Reference(reference) => Types::Reference(reference.clone()),
		}
	}
}

impl Types {
	pub fn assert_matches_type(&self, other: &Types) {
		assert_eq!(std::mem::discriminant(self), std::mem::discriminant(other));
	}

	pub fn transfer_from(&mut self, other: &Types) {
		self.assert_matches_type(other);
		match self {
			Types::Byte(ref mut a) if let Types::Byte(b) = other => a.set(b.get()),
			Types::Short(ref mut a) if let Types::Short(b) = other => a.set(b.get()),
			Types::Int(ref mut a) if let Types::Int(b) = other => a.set(b.get()),
			Types::Long(ref mut a) if let Types::Long(b) = other => a.set(b.get()),
			Types::Char(ref mut a) if let Types::Char(b) = other => a.set(b.get()),
			Types::Float(ref mut a) if let Types::Float(b) = other => a.set(b.get()),
			Types::Double(ref mut a) if let Types::Double(b) = other => a.set(b.get()),
			Types::Boolean(ref mut a) if let Types::Boolean(b) = other => a.set(b.get()),
			Types::ReturnAddress(ref mut a) if let Types::ReturnAddress(b) = other => a.set(b.get()),
			Types::Reference(ref mut a) if let Types::Reference(b) = other => a.set(b.get()),
			_ => panic!("Invalid types transfer")
		}
	}
}

pub trait PrimitiveTypes {}

impl PrimitiveTypes for dyn NumericTypes {}

pub trait NumericTypes {}

impl NumericTypes for dyn IntegralTypes {}
impl NumericTypes for dyn FloatingPointTypes {}

pub trait IntegralTypes {}

pub trait FloatingPointTypes {}

pub trait Value {
	type Type;

	fn new() -> Self where Self: Sized;
	fn from_value(value: Self::Type) -> Self where Self: Sized;
	fn set(&mut self, value: Self::Type);
	fn get(&self) -> Self::Type;
	fn width(&self) -> U2;
}

pub trait Width {
	fn width(&self) -> U2;
}

impl <T> Width for dyn Value<Type = T> {
	fn width(&self) -> U2 {
		self.width()
	}
}

impl Width for Types {
	fn width(&self) -> U2 {
		match self {
			Types::Byte(byte) => byte.width(),
			Types::Short(short) => short.width(),
			Types::Int(int) => int.width(),
			Types::Long(long) => long.width(),
			Types::Char(char) => char.width(),
			Types::Float(float) => float.width(),
			Types::Double(double) => double.width(),
			Types::Boolean(boolean) => boolean.width(),
			Types::ReturnAddress(return_address) => return_address.width(),
			Types::Reference(reference) => reference.width(),
		}
	}
}

pub enum ReferenceableTypes{
	Class(Object),
	Array(Array),
	Interface(Object),
}
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
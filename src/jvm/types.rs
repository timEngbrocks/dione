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
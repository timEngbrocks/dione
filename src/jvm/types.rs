use self::{byte::Byte, short::Short, int::Int, long::Long, char::Char, float::Float, double::Double, boolean::Boolean, return_address::ReturnAddress, reference::Reference, class::Class, array::Array, interface::Interface};

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
pub mod class;
pub mod array;
pub mod interface;

pub enum Types {
	Primitive(PrimitiveTypes),
	Reference(Reference),
}

pub enum ReferenceableTypes {
	Class(Class),
	Array(Array),
	Interface(Interface),
}

pub enum PrimitiveTypes {
	Numeric(NumericTypes),
	Boolean(Boolean),
	ReturnAddress(ReturnAddress),
}

pub enum NumericTypes {
	Integral(IntegralTypes),
	FloatingPoint(FloatingPointTypes),
}

pub enum IntegralTypes {
	Byte(Byte),
	Short(Short),
	Int(Int),
	Long(Long),
	Char(Char),
}

pub enum FloatingPointTypes {
	Float(Float),
	Double(Double),
}

pub trait Value<T> {
	fn new() -> Self;
	fn from_value(value: T) -> Self;
	fn set(&mut self, value: T);
	fn get(&self) -> T;
}
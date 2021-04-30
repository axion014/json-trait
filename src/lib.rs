#![feature(generic_associated_types, associated_type_bounds)]

use cc_traits::{Get, GetMut};

mod object;
mod array;
mod impls;

pub use object::{Object, MutableObject};
pub use array::{Array, MutableArray};

pub trait ForeignJson<'a>: 'a + Sized + PartialEq {
	type Object: Object<'a, Self>;
	type Array: Array<'a, Self>;

	fn as_enum<'b>(&'b self) -> TypedJson<'a, 'b, Self>;

	fn get_attr<'b>(&'b self, k: &str) -> Option<&Self> where 'a: 'b {
        self.as_object().and_then(|a| a.get(k))
    }

	fn get_index<'b>(&'b self, i: usize) -> Option<&Self> where 'a: 'b {
		self.as_array().map(|a| &a[i])
	}

	fn as_object<'b>(&'b self) -> Option<&Self::Object>;
	fn as_array<'b>(&'b self) -> Option<&Self::Array>;
    fn as_number(&self) -> Option<Option<f64>>;
    fn as_string(&self) -> Option<&str>;
	fn as_bool(&self) -> Option<bool>;

	fn is_null(&self) -> bool;
}

pub trait ForeignMutableJson<'a>: ForeignJson<
	'a,
	Object: MutableObject<'a, Self>,
	Array: MutableArray<'a, Self>
> {
	fn get_attr_mut<'b>(&'b mut self, k: &str) -> Option<&mut Self> where 'a: 'b {
        self.as_object_mut().and_then(|a| a.get_mut(k))
    }

	fn get_index_mut<'b>(&'b mut self, i: usize) -> Option<&mut Self> where 'a: 'b {
		self.as_array_mut().map(|a| &mut a[i])
	}

	fn as_object_mut<'b>(&'b mut self) -> Option<&mut Self::Object>;
	fn as_array_mut<'b>(&'b mut self) -> Option<&mut Self::Array>;
}

pub trait BuildableJson<'a>:
	ForeignJson<'a, Object: Clone, Array: Clone>
	+ From<i8>
	+ From<i16>
	+ From<i32>
	+ From<i64>
	+ From<isize>
	+ From<u8>
	+ From<u16>
	+ From<u32>
	+ From<u64>
	+ From<usize>
	+ From<f32>
	+ From<f64>
	+ From<bool>
	+ From<String>
	+ From<&'a str>
	+ From<<Self as ForeignJson<'a>>::Object>
	+ From<<Self as ForeignJson<'a>>::Array>
	+ Clone
{
	type ParseError: std::error::Error + 'static;

	fn from_str(s: &str) -> Result<Self, Self::ParseError>;

	fn empty_object() -> Self;

	fn empty_array() -> Self;

	fn null() -> Self;
}

pub enum TypedJson<'a: 'b, 'b, T: ForeignJson<'a>> {
	Object(&'b T::Object),
	Array(&'b T::Array),
	Number(Option<f64>),
    String(&'b String),
	Null,
    Bool(bool)
}
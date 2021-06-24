#![feature(generic_associated_types, associated_type_bounds)]

use cc_traits::{Get, GetMut};

mod array;
mod impls;
mod object;

pub use array::{Array, MutableArray};
pub use object::{MutableObject, Object};

pub use inline_json::json;

pub trait ForeignJson: Sized + PartialEq + std::fmt::Debug {
	type Object: Object<Self>;
	type Array: Array<Self>;

	fn as_enum(&self) -> typed_json::Borrowed<'_, Self>;

	fn get_attr<'a>(&'a self, k: &str) -> Option<&Self> {
		self.as_object().and_then(|a| a.get(k))
	}

	fn get_index<'a>(&'a self, i: usize) -> Option<&Self> {
		self.as_array().map(|a| &a[i])
	}

	fn as_object(&self) -> Option<&Self::Object>;
	fn as_array(&self) -> Option<&Self::Array>;
	fn as_number(&self) -> Option<Option<f64>>;
	fn as_string(&self) -> Option<&str>;
	fn as_bool(&self) -> Option<bool>;

	fn is_null(&self) -> bool;
}

pub trait ForeignMutableJson: ForeignJson<Object: MutableObject<Self>, Array: MutableArray<Self>> {
	fn as_enum_mut(&mut self) -> typed_json::Mutable<'_, Self>;

	fn get_attr_mut<'a>(&'a mut self, k: &str) -> Option<&mut Self> {
		self.as_object_mut().and_then(|a| a.get_mut(k))
	}

	fn get_index_mut<'a>(&'a mut self, i: usize) -> Option<&mut Self> {
		self.as_array_mut().map(|a| &mut a[i])
	}

	fn as_object_mut(&mut self) -> Option<&mut Self::Object>;
	fn as_array_mut(&mut self) -> Option<&mut Self::Array>;

	fn into_enum(self) -> typed_json::Owned<Self>;

	fn into_object(self) -> Option<Self::Object>;
	fn into_array(self) -> Option<Self::Array>;
	fn into_string(self) -> Option<String>;
}

pub trait BuildableJson:
	ForeignJson<Object: Clone, Array: Clone>
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
	+ for<'a> From<&'a str>
	+ From<<Self as ForeignJson>::Object>
	+ From<<Self as ForeignJson>::Array>
	+ std::str::FromStr<Err: std::error::Error + Send + Sync + 'static>
	+ Clone
{
	fn empty_object() -> Self::Object;

	fn empty_array() -> Self::Array;

	fn null() -> Self;
}

pub mod typed_json {
	use crate::{BuildableJson, ForeignJson, ForeignMutableJson};

	pub enum Borrowed<'a, T: ForeignJson> {
		Object(&'a T::Object),
		Array(&'a T::Array),
		Number(Option<f64>),
		String(&'a String),
		Null,
		Bool(bool)
	}

	pub enum Mutable<'a, T: ForeignMutableJson> {
		Object(&'a mut T::Object),
		Array(&'a mut T::Array),
		Number(Option<f64>),
		String(&'a mut String),
		Null,
		Bool(bool)
	}

	pub enum Owned<T: ForeignMutableJson> {
		Object(T::Object),
		Array(T::Array),
		Number(Option<f64>),
		String(String),
		Null,
		Bool(bool)
	}

	impl<T: ForeignMutableJson + BuildableJson> Owned<T> {
		pub fn into_untyped(self) -> T {
			match self {
				Owned::Object(v) => v.into(),
				Owned::Array(v) => v.into(),
				Owned::Number(v) => v.map_or(T::null(), |v| v.into()),
				Owned::String(v) => v.into(),
				Owned::Null => T::null(),
				Owned::Bool(v) => v.into()
			}
		}
	}
}

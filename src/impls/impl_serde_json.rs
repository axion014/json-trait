use serde_json::{Map, Value};

use crate::typed_json::{self, *};
use crate::{BuildableJson, ForeignJson, ForeignMutableJson, MutableObject, Object};

impl ForeignJson for Value {
	type Object = Map<String, Self>;
	type Array = Vec<Self>;

	fn as_enum(&self) -> typed_json::Borrowed<'_, Self> {
		match self {
			Value::Object(v) => Borrowed::Object(v),
			Value::Array(v) => Borrowed::Array(v),
			Value::Number(v) => Borrowed::Number(v.as_f64()),
			Value::String(v) => Borrowed::String(v),
			Value::Null => Borrowed::Null,
			Value::Bool(v) => Borrowed::Bool(*v)
		}
	}

	fn get_attr<'a>(&'a self, k: &str) -> Option<&Self> {
		self.get(k)
	}

	fn get_index<'a>(&'a self, i: usize) -> Option<&Self> {
		self.get(i)
	}

	fn as_object(&self) -> Option<&Map<String, Self>> {
		self.as_object()
	}

	fn as_array(&self) -> Option<&Vec<Self>> {
		self.as_array()
	}

	fn as_number(&self) -> Option<Option<f64>> {
		if self.is_number() {
			Some(self.as_f64())
		} else {
			None
		}
	}

	fn as_string(&self) -> Option<&str> {
		self.as_str()
	}

	fn as_bool(&self) -> Option<bool> {
		self.as_bool()
	}

	fn is_null(&self) -> bool {
		self.is_null()
	}
}

impl BuildableJson for Value {
	fn empty_object() -> <Value as ForeignJson>::Object {
		Map::new()
	}

	fn empty_array() -> <Value as ForeignJson>::Array {
		Vec::new()
	}

	fn null() -> Self {
		Value::Null
	}
}

impl ForeignMutableJson for Value {
	fn as_enum_mut(&mut self) -> typed_json::Mutable<'_, Self> {
		match self {
			Value::Object(v) => Mutable::Object(v),
			Value::Array(v) => Mutable::Array(v),
			Value::Number(v) => Mutable::Number(v.as_f64()),
			Value::String(v) => Mutable::String(v),
			Value::Null => Mutable::Null,
			Value::Bool(v) => Mutable::Bool(*v)
		}
	}

	fn get_attr_mut<'a>(&'a mut self, k: &str) -> Option<&mut Self> {
		self.get_mut(k)
	}

	fn get_index_mut<'a>(&'a mut self, i: usize) -> Option<&mut Self> {
		self.get_mut(i)
	}

	fn as_object_mut(&mut self) -> Option<&mut Map<String, Self>> {
		self.as_object_mut()
	}

	fn as_array_mut(&mut self) -> Option<&mut Vec<Self>> {
		self.as_array_mut()
	}

	fn into_enum(self) -> typed_json::Owned<Self> {
		match self {
			Value::Object(v) => Owned::Object(v),
			Value::Array(v) => Owned::Array(v),
			Value::Number(v) => Owned::Number(v.as_f64()),
			Value::String(v) => Owned::String(v),
			Value::Null => Owned::Null,
			Value::Bool(v) => Owned::Bool(v)
		}
	}

	fn into_object(self) -> Option<Map<String, Self>> {
		match self {
			Value::Object(object) => Some(object),
			_ => None
		}
	}

	fn into_array(self) -> Option<Vec<Self>> {
		match self {
			Value::Array(array) => Some(array),
			_ => None
		}
	}

	fn into_string(self) -> Option<String> {
		match self {
			Value::String(string) => Some(string),
			_ => None
		}
	}
}

fn with_str_key<'a>(entry: (&'a String, &'a Value)) -> (&'a str, &'a Value) {
	(entry.0.as_str(), entry.1)
}

fn with_str_key_mut<'a>(entry: (&'a String, &'a mut Value)) -> (&'a str, &'a mut Value) {
	(entry.0.as_str(), entry.1)
}

impl Object<Value> for Map<String, Value> {
	type Iter<'a> = std::iter::Map<serde_json::map::Iter<'a>, fn((&'a String, &'a Value)) -> (&'a str, &'a Value)>;

	fn iter(&self) -> Self::Iter<'_> {
		self.iter().map(with_str_key)
	}
}

impl MutableObject<Value> for Map<String, Value> {
	type IterMut<'a> = std::iter::Map<serde_json::map::IterMut<'a>, fn((&'a String, &'a mut Value)) -> (&'a str, &'a mut Value)>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut().map(with_str_key_mut)
	}
}

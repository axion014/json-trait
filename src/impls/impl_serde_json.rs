use std::str::FromStr;

use serde_json::{Value, Map};

use crate::{ForeignJson, BuildableJson, ForeignMutableJson, TypedJson, Object, MutableObject};

impl <'a> ForeignJson<'a> for Value {
	type Object = Map<String, Self>;
	type Array = Vec<Self>;

	fn as_enum<'b>(&'b self) -> TypedJson<'a, 'b, Self> {
		match self {
			Value::Object(v) => TypedJson::Object(v),
			Value::Array(v) => TypedJson::Array(v),
			Value::Number(v) => TypedJson::Number(v.as_f64()),
			Value::String(v) => TypedJson::String(v),
			Value::Null => TypedJson::Null,
			Value::Bool(v) => TypedJson::Bool(*v)
		}
	}

	fn get_attr<'b>(&'b self, k: &str) -> Option<&Self> where 'a: 'b {
		self.get(k)
	}

	fn get_index<'b>(&'b self, i: usize) -> Option<&Self> where 'a: 'b {
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

impl BuildableJson<'_> for Value {
	type ParseError = serde_json::Error;

	fn from_str(s: &str) -> Result<Self, Self::ParseError> {
	    <Value as FromStr>::from_str(s)
	}

	fn empty_object() -> Self {
	    Value::Object(Map::new())
	}

	fn empty_array() -> Self {
		Value::Array(Vec::new())
	}

	fn null() -> Self {
		Value::Null
	}
}

impl <'a> ForeignMutableJson<'a> for Value {
	fn get_attr_mut<'b>(&'b mut self, k: &str) -> Option<&mut Self> where 'a: 'b {
		self.get_mut(k)
	}

	fn get_index_mut<'b>(&'b mut self, i: usize) -> Option<&mut Self> where 'a: 'b {
		self.get_mut(i)
	}

	fn as_object_mut(&mut self) -> Option<&mut Map<String, Self>> {
		self.as_object_mut()
	}

	fn as_array_mut(&mut self) -> Option<&mut Vec<Self>> {
		self.as_array_mut()
	}
}

fn with_str_key<'a>(entry: (&'a String, &'a Value)) -> (&'a str, &'a Value) {
	(entry.0.as_str(), entry.1)
}

fn with_str_key_mut<'a>(entry: (&'a String, &'a mut Value)) -> (&'a str, &'a mut Value) {
	(entry.0.as_str(), entry.1)
}

impl <'a> Object<'a, Value> for Map<String, Value> {
	type Iter<'b> where 'a: 'b =
		std::iter::Map<serde_json::map::Iter<'b>, fn((&'b String, &'b Value)) -> (&'b str, &'b Value)>;

	fn iter<'b>(&'b self) -> Self::Iter<'b> where 'a: 'b {
		self.iter().map(with_str_key)
	}
}

impl <'a> MutableObject<'a, Value> for Map<String, Value> {
	type IterMut<'b> where 'a: 'b =
		std::iter::Map<serde_json::map::IterMut<'b>, fn((&'b String, &'b mut Value)) -> (&'b str, &'b mut Value)>;

	fn iter_mut<'b>(&'b mut self) -> Self::IterMut<'b> where 'a: 'b {
		self.iter_mut().map(with_str_key_mut)
	}
}
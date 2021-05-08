use serde_json::{Value, Map};

use crate::{ForeignJson, BuildableJson, ForeignMutableJson, TypedJson, Object, MutableObject};

impl ForeignJson for Value {
	type Object = Map<String, Self>;
	type Array = Vec<Self>;

	fn as_enum(&self) -> TypedJson<'_, Self> {
		match self {
			Value::Object(v) => TypedJson::Object(v),
			Value::Array(v) => TypedJson::Array(v),
			Value::Number(v) => TypedJson::Number(v.as_f64()),
			Value::String(v) => TypedJson::String(v),
			Value::Null => TypedJson::Null,
			Value::Bool(v) => TypedJson::Bool(*v)
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

impl ForeignMutableJson for Value {
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
}

fn with_str_key<'a>(entry: (&'a String, &'a Value)) -> (&'a str, &'a Value) {
	(entry.0.as_str(), entry.1)
}

fn with_str_key_mut<'a>(entry: (&'a String, &'a mut Value)) -> (&'a str, &'a mut Value) {
	(entry.0.as_str(), entry.1)
}

impl Object<Value> for Map<String, Value> {
	type Iter<'a> =
		std::iter::Map<serde_json::map::Iter<'a>, fn((&'a String, &'a Value)) -> (&'a str, &'a Value)>;

	fn iter(&self) -> Self::Iter<'_> {
		self.iter().map(with_str_key)
	}
}

impl MutableObject<Value> for Map<String, Value> {
	type IterMut<'a> =
		std::iter::Map<serde_json::map::IterMut<'a>, fn((&'a String, &'a mut Value)) -> (&'a str, &'a mut Value)>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut().map(with_str_key_mut)
	}
}
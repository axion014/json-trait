use serde_json::Value;
use crate::{ForeignJson, BuildableJson, ForeignMutableJson, TypedJson, TypedMutableJson, JsonObject};

impl ForeignJson for Value {
	type Ref = &Self;
	type Array = Vec<Self>;

	fn as_enum(&self) -> TypedJson<Self, Self::Array> {
		match self {
			Value::Object(v) => TypedJson::Object(v),
			Value::Array(v) => TypedJson::Array(v),
			Value::Number(v) => TypedJson::Number(v),
			Value::String(v) => TypedJson::String(v),
			Value::Null => TypedJson::Null,
			Value::Bool(v) => TypedJson::Bool(v)
		}
	}

	fn get_index(&self, i: usize) -> Option<&Self> {
		self.get(i)
	}

	fn as_object(&self) -> Option<&JsonObject<Self>> {
		self.as_object()
	}

	fn as_array(&self) -> Option<Self::Array> {
		self.as_array()
	}

    fn as_number(&self) -> Option<f64> {
		self.as_f64()
	}

    fn as_string(&self) -> Option<&str> {
		self.as_string()
	}

	fn as_bool(&self) -> Option<bool> {
		self.as_bool();
	}

	fn is_null(&self) -> bool {
		self.is_null()
	}
}

impl BuildableJson for Value {
	fn null() -> Self {
		Value::Null
	}
}

impl ForeignMutableJson for Value {
	fn as_enum_mut(&mut self) -> TypedMutableJson<Self> {
		match self {
			Value::Object(v) => TypedMutableJson::Object(v),
			Value::Array(v) => TypedMutableJson::Array(v),
			Value::Number(v) => TypedMutableJson::Number(v),
			Value::String(v) => TypedMutableJson::String(v),
			Value::Null => TypedMutableJson::Null,
			Value::Bool(v) => TypedMutableJson::Bool(v)
		}
	}

	fn get_index_mut(&mut self, i: usize) -> Option<&mut Self> {
		self.get_mut(i)
	}

	fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Self>> {
		self.as_object_mut()
	}

	fn as_array_mut(&mut self) -> Option<Self::Array> {
		self.as_array_mut()
	}
}
use std::collections::HashMap;

pub type JsonObject<T> = HashMap<String, T>;

pub trait ForeignJson: Sized {
	type Ref: AsRef<Self>;
	type Array: ExactSizeIterator<Item = Self::Ref>;

	fn as_enum(&self) -> TypedJson<Self, Self::Array>;

	fn get_attr(&self, k: &str) -> Option<&Self> {
        self.as_object().and_then(|a| a.get(k))
    }

	fn get_index(&self, i: usize) -> Option<&Self>;

	fn as_object(&self) -> Option<&JsonObject<Self>>;
	fn as_array(&self) -> Option<Self::Array>;
    fn as_number(&self) -> Option<f64>;
    fn as_string(&self) -> Option<&str>;
	fn as_bool(&self) -> Option<bool>;

	fn is_null(&self) -> bool;
}

pub trait ForeignMutableJson: ForeignJson {
	fn as_enum_mut(&mut self) -> TypedMutableJson<Self>;

	fn get_attr_mut(&mut self, k: &str) -> Option<&mut Self> {
        self.as_object_mut().and_then(|a| a.get_mut(k))
    }

	fn get_index_mut(&mut self, i: usize) -> Option<&mut Self>;

	fn as_object_mut(&mut self) -> Option<&mut JsonObject<Self>>;
	fn as_array_mut(&mut self) -> Option<Self::Array>;
}

pub trait BuildableJson<'a>:
	ForeignJson
	+ From<i8>
	+ From<i16>
	+ From<i32>
	+ From<i64>
	+ From<u8>
	+ From<u16>
	+ From<u32>
	+ From<u64>
	+ From<f32>
	+ From<f64>
	+ From<bool>
	+ From<String>
	+ From<&'a str>
	+ From<JsonObject<Self>>
	+ From<<Self as ForeignJson>::Array>
{
	fn null() -> Self;
}

pub enum TypedJson<'a, T: ForeignJson, A: ExactSizeIterator<Item = T::Ref>> {
	Object(&'a JsonObject<T>), // Map trait pending.
	Array(A),
	Number(f64),
    String(String),
	Null,
    Bool(bool)
}

pub enum TypedMutableJson<T: ForeignMutableJson> {
	Object(JsonObject<T>),
	Array(Vec<T>), // List trait also pending.
	Number(f64),
    String(String),
	Null,
    Bool(bool)
}
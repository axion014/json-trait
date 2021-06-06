use std::ops::Index;
use std::iter::FromIterator;
use std::collections::{HashMap, BTreeMap};

use cc_traits::{Collection, Get, GetMut, MapInsert, Remove, Clear, Len};

use crate::{ForeignJson, ForeignMutableJson};

pub trait Object<T: ForeignJson>:
	Collection<Item = T>
	+ for<'a> Get<&'a str>
	+ for<'a> Index<&'a str, Output = T>
	+ Len
	+ PartialEq
	+ FromIterator<(String, T)>
{
	type Iter<'a>: Iterator<Item = (&'a str, &'a T)> where T: 'a;

	fn iter(&self) -> Self::Iter<'_>;
}

pub trait MutableObject<T: ForeignMutableJson>:
	Object<T>
	+ for<'a> GetMut<&'a str>
	+ MapInsert<String>
	+ for<'a> Remove<&'a str>
	+ Clear
	+ IntoIterator<Item = (String, T)>
{
	type IterMut<'a>: Iterator<Item = (&'a str, &'a mut T)> where T: 'a;

	fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

fn with_str_key<'a, T: ForeignJson>(entry: (&'a String, &'a T)) -> (&'a str, &'a T) {
	(entry.0.as_str(), entry.1)
}

fn with_str_key_mut<'a, T: ForeignMutableJson>(entry: (&'a String, &'a mut T)) -> (&'a str, &'a mut T) {
	(entry.0.as_str(), entry.1)
}

impl <T: ForeignJson> Object<T> for HashMap<String, T> {
	type Iter<'a> where T: 'a =
		std::iter::Map<std::collections::hash_map::Iter<'a, String, T>, fn((&'a String, &'a T)) -> (&'a str, &'a T)>;

	fn iter(&self) -> Self::Iter<'_> {
		self.into_iter().map(with_str_key)
	}
}

impl <T: ForeignMutableJson> MutableObject<T> for HashMap<String, T> {
	type IterMut<'a> where T: 'a =
		std::iter::Map<std::collections::hash_map::IterMut<'a, String, T>, fn((&'a String, &'a mut T)) -> (&'a str, &'a mut T)>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.into_iter().map(with_str_key_mut)
	}
}

impl <T: ForeignJson> Object<T> for BTreeMap<String, T> {
	type Iter<'a> where T: 'a =
		std::iter::Map<std::collections::btree_map::Iter<'a, String, T>, fn((&'a String, &'a T)) -> (&'a str, &'a T)>;

	fn iter(&self) -> Self::Iter<'_> {
		self.into_iter().map(with_str_key)
	}
}

impl <T: ForeignMutableJson> MutableObject<T> for BTreeMap<String, T> {
	type IterMut<'a> where T: 'a =
		std::iter::Map<std::collections::btree_map::IterMut<'a, String, T>, fn((&'a String, &'a mut T)) -> (&'a str, &'a mut T)>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.into_iter().map(with_str_key_mut)
	}
}
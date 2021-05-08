use std::ops::{Index, IndexMut};

use cc_traits::{Collection, Get, GetMut, MapInsert, Remove, Clear, Len};

use crate::{ForeignJson, ForeignMutableJson};

pub trait Object<T: ForeignJson>:
	Collection<Item = T>
	+ for<'a> Get<&'a str>
	+ for<'a> Index<&'a str, Output = T>
	+ Len
	+ PartialEq
{
	type Iter<'a>: Iterator<Item = (&'a str, &'a T)> where T: 'a;

	fn iter(&self) -> Self::Iter<'_>;
}

pub trait MutableObject<T: ForeignMutableJson>:
	Object<T>
	+ for<'a> GetMut<&'a str>
	+ for<'a> IndexMut<&'a str, Output = T>
	+ MapInsert<String>
	+ for<'a> Remove<&'a str>
	+ Clear
	+ IntoIterator<Item = (String, T)>
{
	type IterMut<'a>: Iterator<Item = (&'a str, &'a mut T)> where T: 'a;

	fn iter_mut(&mut self) -> Self::IterMut<'_>;
}
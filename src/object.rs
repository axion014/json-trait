use std::ops::{Index, IndexMut};

use cc_traits::{Collection, Get, GetMut, MapInsert, Remove, Clear, Len};

use crate::{ForeignJson, ForeignMutableJson};

pub trait Object<'a, T: ForeignJson<'a>>:
	Collection<Item = T>
	+ for<'b> Get<&'b str>
	+ for<'b> Index<&'b str, Output = T>
	+ Len
	+ PartialEq
{
	type Iter<'b>: Iterator<Item = (&'b str, &'b T)> where 'a: 'b;

	fn iter<'b>(&'b self) -> Self::Iter<'b> where 'a: 'b;
}

pub trait MutableObject<'a, T: ForeignMutableJson<'a>>:
	Object<'a, T>
	+ for<'b> GetMut<&'b str>
	+ for<'b> IndexMut<&'b str, Output = T>
	+ MapInsert<String>
	+ for<'b> Remove<&'b str>
	+ Clear
	+ IntoIterator<Item = (String, T)>
{
	type IterMut<'b>: Iterator<Item = (&'b str, &'b mut T)> where 'a: 'b;

	fn iter_mut<'b>(&'b mut self) -> Self::IterMut<'b> where 'a: 'b;
}
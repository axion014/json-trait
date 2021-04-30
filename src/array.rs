use std::ops::{Index, IndexMut};

use cc_traits::{Collection,/* MapInsert,*/ Remove, Clear, Len};

use crate::{ForeignJson, ForeignMutableJson};

pub trait Array<'a, T: ForeignJson<'a>>:
	Collection<Item = T>
	+ Index<usize, Output = T>
	+ Len
	+ PartialEq
{
	type Iter<'b>: Iterator<Item = &'b T> where 'a: 'b;

	fn iter<'b>(&'b self) -> Self::Iter<'b> where 'a: 'b;
}

pub trait MutableArray<'a, T: ForeignMutableJson<'a>>:
	Array<'a, T>
	+ IndexMut<usize, Output = T>
	//+ MapInsert<usize>
	+ Remove<usize>
	+ Clear
	+ IntoIterator<Item = T>
{
	type IterMut<'b>: Iterator<Item = &'b mut T> where 'a: 'b;

	fn iter_mut<'b>(&'b mut self) -> Self::IterMut<'b> where 'a: 'b;
}

impl <'a, T: ForeignJson<'a>> Array<'a, T> for Vec<T> {
	type Iter<'b> where 'a: 'b = std::slice::Iter<'b, T>;

	fn iter<'b>(&'b self) -> Self::Iter<'b> where 'a: 'b {
		self.into_iter()
	}
}

impl <'a, T: ForeignMutableJson<'a>> MutableArray<'a, T> for Vec<T> {
	type IterMut<'b> where 'a: 'b = std::slice::IterMut<'b, T>;

	fn iter_mut<'b>(&'b mut self) -> Self::IterMut<'b> where 'a: 'b {
		self.into_iter()
	}
}
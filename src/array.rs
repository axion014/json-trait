use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use cc_traits::{Clear, Collection, Len, PopBack, PushBack, Remove};

use crate::{ForeignJson, ForeignMutableJson};

pub trait Array<T: ForeignJson>: Collection<Item = T> + Index<usize, Output = T> + Len + PartialEq + FromIterator<T> + std::fmt::Debug {
	type Iter<'a>: Iterator<Item = &'a T>
	where
		Self: 'a,
		T: 'a;

	fn iter(&self) -> Self::Iter<'_>;
}

pub trait MutableArray<T: ForeignMutableJson>:
	Array<T>
	+ IndexMut<usize, Output = T>
	//+ MapInsert<usize>
	+ PushBack
	+ PopBack
	+ Remove<usize>
	+ Clear
	+ Extend<T>
	+ IntoIterator<Item = T>
{
	type IterMut<'a>: Iterator<Item = &'a mut T>
	where
		Self: 'a,
		T: 'a;

	fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

impl<T: ForeignJson> Array<T> for Vec<T> {
	type Iter<'a>
	where
		T: 'a
	= std::slice::Iter<'a, T>;

	fn iter(&self) -> Self::Iter<'_> {
		self.into_iter()
	}
}

impl<T: ForeignMutableJson> MutableArray<T> for Vec<T> {
	type IterMut<'a>
	where
		T: 'a
	= std::slice::IterMut<'a, T>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.into_iter()
	}
}

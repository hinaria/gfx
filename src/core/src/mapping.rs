#![deny(missing_docs, missing_copy_implementations)]

//! Memory mapping

use std::ops;
use Backend;

// TODO
/// Error accessing a mapping.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Error;

/// Mapping reader
pub struct Reader<'a, B: Backend, T: 'a + Copy> {
    pub(crate) slice: &'a [T],
    pub(crate) _mapping: B::Mapping,
}

impl<'a, B: Backend, T: 'a + Copy> ops::Deref for Reader<'a, B, T> {
    type Target = [T];

    fn deref(&self) -> &[T] { self.slice }
}

/// Mapping writer.
/// Currently is not possible to make write-only slice so while it is technically possible
/// to read from Writer, it will lead to an undefined behavior. Please do not read from it.
pub struct Writer<'a, B: Backend, T: 'a + Copy> {
    pub(crate) slice: &'a mut [T],
    pub(crate) _mapping: B::Mapping,
}

impl<'a, B: Backend, T: 'a + Copy> ops::Deref for Writer<'a, B, T> {
    type Target = [T];

    fn deref(&self) -> &[T] { self.slice }
}

impl<'a, B: Backend, T: 'a + Copy> ops::DerefMut for Writer<'a, B, T> {
    fn deref_mut(&mut self) -> &mut [T] { self.slice }
}


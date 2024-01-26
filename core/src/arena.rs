use std::fmt::{Debug, Formatter};
use std::marker::{PhantomData, Unsize};
use std::ops;

pub struct Arena<T>
    where T: ?Sized,
{
    elements: Vec<Box<T>>,
    //phantom: PhantomData<&'arena ()>,
}

#[allow(dead_code)]
impl<T> Arena<T>
    where T: ?Sized,
{
    pub fn new() -> Self {
        return Self {
            elements: Vec::new(),
        };
    }

    /// Returns the number of elements stored in the arena
    pub fn len(&self) -> usize {
        return self.elements.len();
    }

    pub fn append<E>(&mut self, element: E) -> Ref<E, T>
        where E: Unsize<T>,
    {
        let idx = self.elements.len();
        self.elements.push(Box::<E>::new(element));

        return Ref {
            index: idx,
            phantom: PhantomData::default(),
        };
    }

    /// Walk over all elements in the arena  while applying the given callback for each element.
    /// The slice provided in each call is a view to all elements added before the currently
    /// processed element.
    pub fn try_walk<E>(&mut self, f: impl Fn(&mut T, Slice<T>) -> Result<(), E>) -> Result<(), E> {
        for i in 0..self.elements.len() {
            let (lead, tail) = self.elements.split_at_mut(i);
            let curr = &mut tail[0];

            f(curr, Slice {
                offset: 0,
                elements: lead,
            })?;
        }

        return Ok(());
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> + DoubleEndedIterator {
        return self.elements.iter().map(|e| e.as_ref());
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> + DoubleEndedIterator {
        return self.elements.iter_mut().map(|e| e.as_mut());
    }

    pub fn as_slice(&self) -> Slice<T> {
        return Slice {
            offset: 0,
            elements: self.elements.as_slice(),
        };
    }
}

pub struct Slice<'arena, T>
    where T: ?Sized,
{
    offset: usize,
    elements: &'arena [Box<T>],
}

#[allow(dead_code)]
impl<'arena, T> Slice<'arena, T>
    where T: ?Sized,
{
    pub fn len(&self) -> usize {
        return self.elements.len();
    }
}

impl<E, T> ops::Index<&Ref<E, T>> for Slice<'_, T>
    where E: Unsize<T>,
          T: ?Sized,
{
    type Output = E;

    fn index(&self, index: &Ref<E, T>) -> &Self::Output {
        let entry = self.elements[index.index - self.offset].as_ref() as *const T;
        let entry = unsafe { &*(entry as *const E) };
        return entry;
    }
}

pub struct Ref<E, T>
    where E: Unsize<T>,
          T: ?Sized,
{
    index: usize,
    phantom: PhantomData<(E, T)>,
}

impl<E, T> Debug for Ref<E, T>
    where E: Unsize<T>,
          T: ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ref{{index={}}}", self.index)?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::fmt::Debug;

    use anyhow::Result;

    use super::*;

    #[test]
    fn empty() {
        let mut arena = Arena::<()>::new();

        assert_eq!(arena.len(), 0);

        assert_eq!(arena.iter().next(), None);
        assert_eq!(arena.iter_mut().next(), None);

        assert_eq!(arena.as_slice().len(), 0);
    }

    #[test]
    fn basics() {
        let mut arena = Arena::<dyn Obj<u8>>::new();

        arena.append(1_u8);
        arena.append(2_u8);
        arena.append(3_u8);

        assert_eq!(arena.len(), 3);

        assert_eq!(arena.iter().collect::<Vec<_>>(), vec![&1u8, &2u8, &3u8]);
        assert_eq!(arena.iter_mut().collect::<Vec<_>>(), vec![&1u8, &2u8, &3u8]);
    }

    #[test]
    fn index() {
        let mut arena = Arena::<dyn Obj<u8>>::new();

        let ref1 = arena.append(1_u8);
        let ref2 = arena.append(2_u8);
        let ref3 = arena.append(3_u8);

        let slice = arena.as_slice();

        assert_eq!(slice[&ref1], 1u8);
        assert_eq!(slice[&ref2], 2u8);
        assert_eq!(slice[&ref3], 3u8);
    }

    #[test]
    fn walk() {
        let mut arena = Arena::<dyn Obj<u8>>::new();

        let ref1 = arena.append(1_u8);
        let ref2 = arena.append(2_u8);
        let ref3 = arena.append(3_u8);

        let i = RefCell::new(0);

        let _: Result<()> = arena.try_walk(|curr, tail| {
            match *i.borrow() {
                0 => {
                    assert_eq!(curr, &1_u8);
                    assert_eq!(tail.len(), 0);
                }

                1 => {
                    assert_eq!(curr, &2_u8);
                    assert_eq!(tail.len(), 1);
                    assert_eq!(tail[&ref1], 1_u8);
                }

                2 => {
                    assert_eq!(curr, &3_u8);
                    assert_eq!(tail.len(), 2);
                    assert_eq!(tail[&ref1], 1_u8);
                    assert_eq!(tail[&ref2], 2_u8);
                }

                _ => panic!("To many calls")
            }

            i.replace_with(|&mut i| i + 1);

            return Ok(());
        });
    }

    trait Obj<T>: PartialEq<T> + Debug
        where T: Clone,
    {}

    impl<T> Obj<T> for T
        where T: PartialEq<T> + Debug + Clone,
    {}
}
